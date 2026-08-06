// @ts-check
(function () {
  "use strict";

  /**DOM 选择器字段 */
  const DOM = Object.freeze({
    courseTree: {
      nodeClass: "div.posCatalog_select",
      titleClass: "span.posCatalog_title",
      activeClass: "posCatalog_active",
      nameClass: "span.posCatalog_name",
      unfinishedClass: ".orangeNew",
    },

    chapter: {
      tabClass: "div.prev_white",
    },

    video: {
      containerId: "video",
      questionContainerId: "ext-comp-1046",
      questionContinueId: "videoquiz-continue",
      questionSubmittingId: "videoquiz-submitting",
      playControlClass: ".vjs-play-control",
      endedClass: "vjs-ended",
      iframeClass: "ans-insertvideo-online",
      launchBtnClass: ".vjs-big-play-button",
      pausedClass: "vjs-paused",
      muteBtnClass: ".vjs-mute-control",
      paceListClass: "li.vjs-menu-item",
      startedClass: "vjs-has-started",
      paceSelectedClass: "vjs-menu-item-selected",
      questionSubmitClass: ".ans-videoquiz-submit",
      questionRadiosClass: '.tkItem_ul .ans-videoquiz-opt input[type="radio"]',
      questionCheckboxesClass:
        '.tkItem_ul .ans-videoquiz-opt input[type="checkbox"]',
    },

    pdf: {
      iframeId: "#panView",
      docClass: "insertdoc-online-pdf",
    },

    frames: {
      blankUrl: "about:blank",
      nextBtnId: "prevNextFocusNext",
      workPath: "/ananas/modules/work/",
      jobIconSelector: ".ans-job-icon",
      chapterFrameId: "#iframe", // Layer 2: 章节主框架 ID (旧 outerId)
      taskFrameSelector: "iframe.ans-attach-online", // Layer 3: 任务卡片框架选择器 (旧 innerCourseId)
      taskFrameClass: "ans-attach-online", // Layer 3: 任务卡片框架 Class (旧 innerCourseClass)
      mainContentClass: ".content",
    },
  });

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
  const wait = Object.freeze({
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
     * @returns {Promise<HTMLElement | undefined>}
     */
    element: async (preAction, selector, root) =>
      (await wait.elements(preAction, selector, root))[0],

    /**
     * 等待并获取就绪的 iframe Document（自动过滤 about:blank 阶段及旧 Document 引用）
     * @param {(() => void | Promise<void>) | null} preAction 在等待前执行的触发动作（无触发动作传 null）
     * @param {string} selector iframe 的选择器
     * @param {ParentNode} root 根查找节点
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

      const isDone = () => container.classList.contains("ans-job-finished");
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
  });

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
   *  @type {<T = any>(cmd: string, args?: Record<string, any>) => Promise<T>} */
  const tauriInvoke = /** @type {any} */ (globalThis).__TAURI_INTERNALS__
    ?.invoke;

  /** @param {number} ms */
  const sleep = (ms) =>
    new Promise((resolve) => globalThis.setTimeout(resolve, ms));

  class AppConfig {
    hasBackend = false;
    muteVideo = true;
    lockingSpeed = false;
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
    chapterNodes: /** @type {HTMLElement[]} */ ([]),
    quizAnswerCache: [],
  };

  /**
   * @param {Document} document
   * @returns {HTMLElement[]}
   */
  const chapterNodes = (document) => {
    const nodes = /** @type {HTMLElement[]} */ (
      Array.from(document.querySelectorAll(DOM.courseTree.nodeClass))
    );
    if (nodes.length > 0) {
      console.info("获取课程列表成功：", nodes);
    } else {
      console.error("获取课程列表失败");
    }
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
      if (url.includes("/ananas/modules/video/")) return "Video";
      if (url.includes("/ananas/modules/pdf/")) return "PDF";
      if (url.includes("/ananas/modules/work/")) return "Quiz";
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
      await wait.element(null, "video", taskDoc)
    );
    if (state.config.muteVideo) {
      muteVideo(videoEl);
    }

    const isStarted = await wait.until(() => {
      if (videoEl?.currentTime > 0 && !videoEl.paused) {
        return true;
      }
      launchBtn?.click();
      return null;
    });

    if (!isStarted) {
      console.error("视频多次尝试无法启动播放，跳过该任务点");
      throw new Error("视频多次尝试无法启动播放");
    }

    if (state.config.lockingSpeed) {
      applySpeed(videoEl, state.config.videoSpeedValue);
    }
    console.info("Video任务点处理完成");
  };

  /**
   * 处理 PDF 任务点
   * @param {Document} taskDoc
   */
  const handlePDF = async (taskDoc) => {
    console.info("开始处理PDF任务点");
    const pdfDoc = await wait.iframeDoc(null, DOM.pdf.iframeId, taskDoc);
    if (!pdfDoc) {
      throw new Error("获取 PDF 文档框架失败");
    }
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
    if (!isScrolled) {
      throw new Error("多次尝试后 PDF 仍未滚动");
    }
    console.info("PDF任务点处理完成");
  };

  /**
   * @param {Document} chapterDoc
   */
  const handleTab = async (chapterDoc) => {
    const containers = await wait.elements(
      null,
      ".ans-attach-ct:has(.ans-job-icon)",
      chapterDoc,
    );
    for (const [index, container] of containers.entries()) {
      const taskInfo = `第 ${index + 1}/${containers.length} 个任务点`;
      console.info(taskInfo);

      const taskIframe = /** @type {HTMLIFrameElement | null} */ (
        await wait.element(null, "iframe", container)
      );
      const taskDoc = await wait.until(() => getReadyIframeDoc(taskIframe));

      if (container.classList.contains("ans-job-finished")) {
        console.info(`${taskInfo} 已完成，自动跳过`);
        continue;
      }

      const taskHandler = {
        Video: handleVideo,
        PDF: handlePDF,
        // Quiz: async () => {},
      };
      const type = classifyTask(taskDoc);
      const handler = taskHandler[type];

      if (!handler) {
        console.warn(
          `${taskInfo} (不支持的类型: ${type}) ${taskDoc?.location?.href}`,
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
   * @param {HTMLElement} node
   */
  const handleChapter = async (node) => {
    /** @type {HTMLElement | null} */
    const nameSpan = node.querySelector(DOM.courseTree.nameClass);
    console.info(`开始进入章节[${nameSpan?.getAttribute("title")}]`);

    // 单页章节依然保留了隐藏的 Tab 元素
    for (const tab of await wait.elements(
      () => nameSpan?.click(),
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

  const main = async () => {
    await state.config.loadFromBackend(tauriInvoke);
    confirm("开始答题");
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
