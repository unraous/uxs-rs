// @ts-check
(function () {
  "use strict";

  /** DOM 选择器及静态字符串统一配置表 */
  const DOM = {
    courseTree: {
      nodeClass: "div.posCatalog_select",
      titleClass: "span.posCatalog_title",
      nameClass: "span.posCatalog_name",
      unfinishedClass: ".orangeNew",
    },

    chapter: {
      tabClass: "div.prev_white",
    },

    video: {
      tag: "video",
      launchBtnClass: ".vjs-big-play-button",
    },

    pdf: {
      iframeId: "#panView",
    },

    quiz: {
      titleClass: ".newZy_TItle",
      clearSelectedClass: "span:is(.check_answer, .check_answer_dx)",
      blankItemDiv: ".blankItemDiv",
      ueditorIframe: 'iframe[id^="ueditor_"]',
      frameContentId: "#frame_content",
      singleQuesClass: "div.singleQuesId",
      btnSubmitClass: "a.btnSubmit",
      modalId: "#workpop",
      modalOkBtnId: "a#popok",
    },

    taskPoint: {
      finishedClass: "ans-job-finished",
      containerSelector: ".ans-attach-ct:has(.ans-job-icon)",
      iframeTag: "iframe",
    },

    frames: {
      blankUrl: "about:blank",
      chapterFrameId: "#iframe", // Layer 2: 章节主框架 ID (旧 outerId)
    },

    urls: {
      video: "/ananas/modules/video/",
      pdf: "/ananas/modules/pdf/",
      quiz: "/ananas/modules/work/",
    },
  };

  /**
   * 尝试提取已就绪的 iframe Document，若未就绪或跨域异常则返回 null
   * @param {HTMLIFrameElement | null | undefined} iframe
   * @param {Document | null} [prevDoc=null] 需要排除的旧 Document 引用
   * @returns {Document | null}
   */
  const getReadyIframeDoc = (iframe, prevDoc = null) => {
    try {
      const doc = iframe?.contentDocument;
      if (
        doc?.location?.href !== DOM.frames.blankUrl &&
        doc !== prevDoc &&
        doc?.body?.children?.length
      ) {
        return doc;
      }
    } catch (error) {
      console.warn("访问异常，无法获取iframe文档", error);
    }
    return null;
  };

  /**
   * DOM 异步等待工具库
   */
  const wait = {
    /**
     * @template T
     * @param {() => T} getter 元素获取函数
     * @param {number} [timeout=5000] 超时时间(ms)
     * @param {number} [interval=250] 检查间隔(ms)
     * @returns {Promise<T | null>}
     */
    until: async (getter, timeout = 5000, interval = 250) => {
      const start = performance.now(); // ◄◄ 推荐使用高精度单调时钟
      while (performance.now() - start < timeout) {
        const el = await getter();
        if (el) return el;
        await sleep(interval);
      }
      return null;
    },

    /**
     * 等待指定选择器的元素列表出现并返回（非空）
     * @template {HTMLElement} [T=HTMLElement]
     * @param {(() => void | Promise<void>) | null} preAction 在等待前执行的触发动作（无触发动作传 null）
     * @param {string} selector 选择器表达式
     * @param {ParentNode} root 根查找节点
     * @returns {Promise<T[]>}
     */
    elements: async (preAction, selector, root) => {
      if (typeof preAction === "function") {
        await preAction();
      }
      return (
        (await wait.until(() => {
          const nodes = /** @type {T[]} */ (
            Array.from(root.querySelectorAll(selector))
          );
          return nodes.length > 0 ? nodes : null;
        })) ?? []
      );
    },

    /**
     * 等待指定选择器的首个元素出现并返回
     * @param {(() => void | Promise<void>) | null} preAction 在等待前执行的触发动作（无触发动作传 null）
     * @param {string} selector 选择器表达式
     * @param {ParentNode} root 根查找节点
     * @returns {Promise<HTMLElement | null>}
     */
    element: async (preAction, selector, root) => {
      if (typeof preAction === "function") {
        await preAction();
      }
      return await wait.until(
        () => /** @type {HTMLElement | null} */ (root.querySelector(selector)),
      );
    },

    /**
     * 从父级 Document/节点中查找匹配选择器的 iframe，
     * 等待并获取其内部嵌套的就绪子级 Document 对象
     * （自动过滤 about:blank 阶段及旧 Document 引用）
     * @param {(() => void | Promise<void>) | null} preAction 在等待前执行的触发动作（无触发动作传 null）
     * @param {string} selector iframe 的选择器
     * @param {Document} root 根查找节点
     * @returns {Promise<Document | null>}
     */
    iframeDoc: async (preAction, selector, root) => {
      let oldDoc = null;
      if (typeof preAction === "function") {
        oldDoc =
          /** @type {HTMLIFrameElement | null} */ (root.querySelector(selector))
            ?.contentDocument ?? null;
        await preAction();
      }
      return await wait.until(() =>
        getReadyIframeDoc(
          /** @type {HTMLIFrameElement | null} */ (
            root.querySelector(selector)
          ),
          oldDoc,
        ),
      );
    },

    /**
     * 等待指定任务点在 DOM 中更新为完成状态 (绿标对勾)
     * @param {HTMLElement} container 任务点的大容器节点对象
     * @returns {Promise<boolean>}
     */
    taskPointComplete: (container) => {
      console.info("开始监控任务点完成情况");
      if (!container) return Promise.resolve(true);

      const isDone = () =>
        container.classList.contains(DOM.taskPoint.finishedClass);
      if (isDone()) return Promise.resolve(true);

      return new Promise((resolve) => {
        const observer = new MutationObserver(() => {
          if (isDone()) {
            observer.disconnect();
            console.info("任务点在 DOM 中已更新为完成状态");
            resolve(true);
          }
        });
        observer.observe(container, {
          attributes: true,
          attributeFilter: ["class"],
        });
      });
    },
  };

  /**
   * 安全执行异步操作，捕获并记录异常而不中断上层批处理
   * @param {() => any} action 异步操作函数
   * @param {string} errorMessage 异常提示前缀
   */
  const safeRun = async (action, errorMessage) => {
    try {
      await action();
    } catch (e) {
      console.error(errorMessage, e);
    }
  };

  /**
   * tauri backend command function
   * @type {<T = any>(cmd: string, args?: Record<string, any>) => Promise<T>}
   */
  const tauriInvoke = /** @type {any} */ (globalThis).__TAURI_INTERNALS__
    ?.invoke;

  /** @param {number} ms */
  const sleep = (ms) =>
    new Promise((resolve) => globalThis.setTimeout(resolve, ms));

  /** 应用运行配置对象 */
  class AppConfig {
    /** 是否接入后端 (Tauri) */
    hasBackend = false;
    /** 是否对视频启用静音 */
    muteVideo = true;
    /** 是否锁定视频播放倍速 */
    lockingSpeed = false;
    /** 目标倍速数值 */
    videoSpeedValue = 2.0;

    /**
     * 异步从后端拉取配置并更新字段
     * @param {<T = any>(cmd: string, args?: Record<string, any>) => Promise<T>} invoke
     */
    async loadFromBackend(invoke) {
      if (!invoke) {
        console.info("检测为无后端模式，使用默认配置");
        return;
      }
      try {
        console.info("正在从后端加载配置...");
        const res = await invoke("options");
        if (res) {
          this.hasBackend = true;
          this.muteVideo = res.muteWebview ?? this.muteVideo;
          this.lockingSpeed = res.speedLock ?? this.lockingSpeed;
          this.videoSpeedValue = res.speedValue ?? this.videoSpeedValue;
          console.info("配置设置成功：", this);
        }
      } catch (e) {
        console.error("从后端加载配置失败：", e);
      }
    }
  }

  /**
   * 预先对视频节点做静音处理 (播放前调用)
   * @param {HTMLMediaElement | null} videoEl
   */
  const muteVideo = (videoEl) => {
    if (!videoEl) return;
    videoEl.muted = true;
    videoEl.defaultMuted = true;
  };

  /**
   * 视频真实播放后施加倍速与锁定 (播放后调用)
   * @param {HTMLMediaElement | null} videoEl
   * @param {number} targetRate
   */
  const applySpeed = (videoEl, targetRate) => {
    videoEl.playbackRate = targetRate;
    Object.defineProperty(videoEl, "playbackRate", {
      get: () => targetRate,
      set: () => {},
      configurable: true,
    });
  };

  /** 全局模块共享运行状态单例 */
  const state = {
    config: new AppConfig(),
  };

  /**
   * 获取页面中所有课程章节 DOM 节点列表
   * @param {Document} document
   * @returns {HTMLElement[]}
   */
  const chapterNodes = (document) => {
    const nodes = /** @type {HTMLElement[]} */ (
      Array.from(document.querySelectorAll(DOM.courseTree.nodeClass))
    );
    if (nodes.length > 0) console.info("获取课程列表成功：", nodes);
    else console.error("获取课程列表失败");

    return nodes;
  };

  /**
   * 获取课程章节状态
   * @param {HTMLElement} node
   * @returns {'Blocking' | 'Interactive' | 'Finished' | 'Title' | 'Unknown'}
   */
  const chapterNodeStatus = (node) => {
    /** @type {HTMLElement | null} */
    const nameSpan = node.querySelector(DOM.courseTree.nameClass);
    if (!nameSpan) {
      return node.querySelector(DOM.courseTree.titleClass)
        ? "Title"
        : "Unknown";
    }
    if (nameSpan.onclick == null) {
      return "Blocking";
    }
    return node.querySelector(DOM.courseTree.unfinishedClass)
      ? "Interactive"
      : "Finished";
  };

  /**
   * @param {Document} taskDoc
   * @returns {"Video" | "PDF" | "Quiz" | "Other"}
   */
  const classifyTask = (taskDoc) => {
    try {
      const url = taskDoc.location.href;
      if (url.includes(DOM.urls.video)) return "Video";
      if (url.includes(DOM.urls.pdf)) return "PDF";
      if (url.includes(DOM.urls.quiz)) return "Quiz";
    } catch {
      console.warn("获取任务文档失败，无法识别任务类型");
    }
    return "Other";
  };

  /**
   * 处理视频任务点
   * @param {Document} taskDoc 任务点的文档对象
   */
  const handleVideo = async (taskDoc) => {
    console.info("开始处理Video任务点");
    const launchBtn = await wait.element(
      null,
      DOM.video.launchBtnClass,
      taskDoc,
    );

    const videoEl = /** @type {HTMLMediaElement | null} */ (
      await wait.element(null, DOM.video.tag, taskDoc)
    );
    if (state.config.muteVideo) muteVideo(videoEl);

    const isStarted = await wait.until(() => {
      if (videoEl?.currentTime > 0 && !videoEl.paused) return true;

      launchBtn?.click();
      return null;
    });
    if (!isStarted) throw new Error("视频多次尝试无法启动播放");

    if (state.config.lockingSpeed)
      applySpeed(videoEl, state.config.videoSpeedValue);
    console.info("Video任务点处理完成");
  };

  /**
   * 处理 PDF 任务点
   * @param {Document} taskDoc
   */
  const handlePDF = async (taskDoc) => {
    console.info("开始处理PDF任务点");
    const pdfDoc = await wait.iframeDoc(null, DOM.pdf.iframeId, taskDoc);
    const container = pdfDoc.documentElement;
    const isScrolled = await wait.until(async () => {
      container.scrollTo({
        top: container.scrollHeight,
        behavior: "smooth",
      });
      await sleep(750);
      if (
        container.scrollTop + container.clientHeight >=
        container.scrollHeight - 10
      ) {
        return true;
      }
      return null;
    });

    if (!isScrolled) throw new Error("多次尝试后 PDF 仍未滚动");
    console.info("PDF任务点处理完成");
  };

  /**
   * 清空单个题目 DOM 中的已有勾选/选中状态（针对单选、多选、判断）
   * @param {HTMLElement} quizDiv 题目容器
   */
  const cleanup = (quizDiv) => {
    /** @type {NodeListOf<HTMLElement>} */
    const selectedList = quizDiv.querySelectorAll(DOM.quiz.clearSelectedClass);
    for (const el of selectedList) el.click();
  };

  /**
   * 根据题型分类推导
   * @param {string} titleText
   * @returns {"single" | "multi" | "judge" | "completion" | "short" | null}
   */
  const classifyQuestion = (titleText) => {
    const text = titleText.toLowerCase();
    if (text.includes("多选") || text.includes("mul")) return "multi";
    if (text.includes("判断") || text.includes("true")) return "judge";
    if (text.includes("单选") || text.includes("single")) return "single";
    if (text.includes("填空") || text.includes("fill")) return "completion";
    if (text.includes("简答") || text.includes("short")) return "short";
    return null;
  };

  /**
   * @typedef {Object} AnswerItem
   * @property {string} index 题号
   * @property {string} explanation 解析
   * @property {string} content 答案文本/代号
   */

  /**
   * 单选题填充
   * @param {HTMLElement} quizDiv
   * @param {string} content
   */
  const fillSingle = (quizDiv, content) => {
    /** @type {HTMLElement} */
    const opt = quizDiv.querySelector(
      `span.num_option[data="${String(content).trim()}"]`,
    );
    opt.click();
  };

  /**
   * 多选题填充
   * @param {HTMLElement} quizDiv
   * @param {string | string[]} content
   */
  const fillMulti = (quizDiv, content) => {
    const ansArr = String(content).match(/[A-Z0-9]/gi);

    for (const ch of ansArr) {
      /** @type {HTMLElement} */
      const opt = quizDiv.querySelector(`span.num_option_dx[data="${ch}"]`);
      opt.click();
    }
  };

  /**
   * 判断题填充
   * @param {HTMLElement} quizDiv
   * @param {string | boolean} content
   */
  const fillJudge = (quizDiv, content) => {
    const val = ["A", "对", "t", "T", "true", "1"].includes(
      String(content).trim()[0],
    )
      ? "true"
      : "false";

    /** @type {HTMLElement} */
    const opt = quizDiv.querySelector(`span.num_option[data="${val}"]`);
    opt.click();
  };

  /**
   * 填空题填充（深层直连透传）
   * @param {HTMLElement} quizDiv
   * @param {string} content
   */
  const fillCompletion = (quizDiv, content) => {
    const ansArr = String(content)
      .split(";")
      .map((s) => s.trim());
    for (const [index, itemDiv] of quizDiv
      .querySelectorAll(DOM.quiz.blankItemDiv)
      .entries()) {
      const iframe = /** @type {HTMLIFrameElement | null} */ (
        itemDiv.querySelector(DOM.quiz.ueditorIframe)
      );
      iframe.contentDocument.body.innerHTML = ansArr[index];
    }
  };

  /**
   * 简答题填充
   * @param {HTMLElement} quizDiv
   * @param {string} content
   */
  const fillShort = (quizDiv, content) => {
    const iframe = /** @type {HTMLIFrameElement | null} */ (
      quizDiv.querySelector(DOM.quiz.ueditorIframe)
    );
    iframe.contentDocument.body.innerHTML = String(content);
  };

  /**
   * 为测验题目填充答案
   * @param {HTMLElement[]} quizList 测试题的题目div元素列表
   * @param {AnswerItem[]} answersList 答案列表
   * @returns {Promise<void>}
   */
  const applyAnswers = async (quizList, answersList) => {
    const questionFillers = {
      single: fillSingle,
      multi: fillMulti,
      judge: fillJudge,
      completion: fillCompletion,
      short: fillShort,
    };
    for (const [index, quizDiv] of quizList.entries()) {
      cleanup(quizDiv);
      questionFillers[
        classifyQuestion(
          quizDiv.querySelector(DOM.quiz.titleClass).textContent.toLowerCase(),
        )
      ](quizDiv, answersList[index].content);
    }
  };

  /**
   * 处理 Quiz (测验) 任务点，通过后端 AI 解题、填充答案并提交
   * @param {Document} taskDoc 测验框架的 Document 对象
   * @returns {Promise<void>}
   */
  const handleQuiz = async (taskDoc) => {
    console.info("开始处理Quiz任务点");
    const quizDoc = await wait.iframeDoc(
      null,
      DOM.quiz.frameContentId,
      taskDoc,
    );
    const [quizList, answersList] = await Promise.all([
      wait.elements(null, DOM.quiz.singleQuesClass, quizDoc),
      tauriInvoke("solve_quiz", {
        html: quizDoc.documentElement.outerHTML,
      }),
    ]);
    applyAnswers(quizList, answersList);

    // 1. 点击提交按钮
    (await wait.element(null, DOM.quiz.btnSubmitClass, quizDoc)).click();

    const modal = await wait.element(null, DOM.quiz.modalId, document);
    const popOkBtn = await wait.element(null, DOM.quiz.modalOkBtnId, modal);
    let hasOpened = false;
    await wait.until(() => {
      const isVisible = getComputedStyle(modal).display !== "none";
      if (isVisible) {
        hasOpened = true;
        popOkBtn.click();
        return null;
      }
      return hasOpened ? true : null;
    });

    console.info("已自动填充AI生成的答案并确认提交");
  };

  /**
   * 处理章节页面中包含的所有任务点选项卡
   * @param {Document} chapterDoc 章节主框架的 Document 对象
   * @returns {Promise<void>}
   */
  const handleTab = async (chapterDoc) => {
    const containers = await wait.elements(
      null,
      DOM.taskPoint.containerSelector,
      chapterDoc,
    );
    for (const [index, container] of containers.entries()) {
      const taskInfo = `第 ${index + 1}/${containers.length} 个任务点`;
      const taskIframe = /** @type {HTMLIFrameElement | null} */ (
        await wait.element(null, DOM.taskPoint.iframeTag, container)
      );
      const taskDoc = await wait.until(() => getReadyIframeDoc(taskIframe));
      if (container.classList.contains(DOM.taskPoint.finishedClass)) {
        console.info(`${taskInfo} 已完成，自动跳过`);
        continue;
      }
      console.info(taskInfo);

      const taskHandler = {
        Video: handleVideo,
        PDF: handlePDF,
        Quiz: state.config.hasBackend ? handleQuiz : null, // 依赖tauriInvoke来AI答题
      };
      const type = classifyTask(taskDoc);
      const handler = taskHandler[type];

      if (!handler) {
        console.warn(
          `${taskInfo} (不支持的类型: ${type}) ${taskDoc.location?.href}`,
        );
        continue;
      }
      await safeRun(
        () =>
          Promise.all([handler(taskDoc), wait.taskPointComplete(container)]),
        `${taskInfo} 处理异常，自动跳过，任务点类别：${type}`,
      );
    }
  };

  /**
   * 处理指定章节节点及其嵌套页签
   * @param {HTMLElement} node 章节节点元素
   * @returns {Promise<void>}
   */
  const handleChapter = async (node) => {
    /** @type {HTMLElement | null} */
    const nameSpan = node.querySelector(DOM.courseTree.nameClass);
    console.info(`开始进入章节[${nameSpan.getAttribute("title")}]`);

    // 单页章节依然保留了隐藏的 Tab 元素
    for (const tab of await wait.elements(
      () => nameSpan.click(),
      DOM.chapter.tabClass,
      document,
    )) {
      await safeRun(async () => {
        console.info("等待章节主框架加载");
        const chapterDoc = await wait.iframeDoc(
          () => tab.click(),
          DOM.frames.chapterFrameId,
          document,
        );
        if (!chapterDoc) throw new Error("获取章节主框架超时");
        await handleTab(chapterDoc);
      }, "页签处理失败，自动跳过该页签");
    }
    console.info("本章节处理完毕");
  };

  /**
   * 脚本全流程执行主入口
   * @returns {Promise<void>}
   */
  const main = async () => {
    await state.config.loadFromBackend(tauriInvoke);
    confirm(
      "本脚本仅供学习交流使用, 请遵守相关法律法规。\n\n请先关闭浏览器的开发者工具, 点击确定后单击页面任意处以运行脚本。\n\n如果想停止脚本, 随时刷新页面即可。",
    );
    for (const node of chapterNodes(document)) {
      const status = chapterNodeStatus(node);
      if (status === "Interactive") {
        await safeRun(
          () => handleChapter(node),
          "章节处理失败，自动跳过该章节",
        );
      }
    }
  };

  main();
})();
