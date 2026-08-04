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
      iframeId: "panView",
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
        !doc ||
        doc === prevDoc ||
        doc.location?.href === DOM.frames.blankUrl ||
        !doc.body?.children?.length
      ) {
        return null;
      }
      return doc;
    } catch {
      return null;
    }
  };

  /**
   * DOM 异步等待工具库
   */
  const wait = Object.freeze({
    /**
     * @template T
     * @param {() => T} getter 元素获取函数
     * @param {number} [timeout=10000] 超时时间(ms)
     * @param {number} [interval=200] 检查间隔(ms)
     * @returns {Promise<T | null>}
     */
    until: async (getter, timeout = 10000, interval = 200) => {
      const start = performance.now(); // ◄◄ 推荐使用高精度单调时钟
      while (performance.now() - start < timeout) {
        const el = getter();
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
     * @param {ParentNode} [root] 根查找节点
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
     * @param {ParentNode} [root] 根查找节点
     * @returns {Promise<HTMLElement | null>}
     */
    element: async (preAction, selector, root) => {
      return (await wait.elements(preAction, selector, root))[0] ?? null;
    },

    /**
     * 等待并获取就绪的 iframe Document（自动过滤 about:blank 阶段及旧 Document 引用）
     * @param {(() => void | Promise<void>) | null} preAction 在等待前执行的触发动作（无触发动作传 null）
     * @param {string} selector iframe 的选择器
     * @param {ParentNode} [root=document] 根查找节点
     * @returns {Promise<Document | null>}
     */
    iframeDoc: async (preAction, selector, root) => {
      const oldDoc =
        /** @type {HTMLIFrameElement | null} */ (root.querySelector(selector))
          ?.contentDocument ?? null;

      if (typeof preAction === "function") {
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
  });

  /**
   * tauri backend command function
   *  @type {<T = any>(cmd: string, args?: Record<string, any>) => Promise<T>} */
  const tauriInvoke = /** @type {any} */ (globalThis).__TAURI_INTERNALS__
    ?.invoke;

  /** @param {number} ms */
  const sleep = (ms) =>
    new Promise((resolve) => globalThis.setTimeout(resolve, ms));

  /**
   * @typedef {Object} AppConfig
   * @property {boolean} hasBackend 是否连通后端
   * @property {boolean} muteVideo 是否视频静音
   * @property {boolean} lockingSpeed 是否锁定播放倍速
   * @property {number} videoSpeedValue 播放倍速数值
   */

  /**
   * @typedef {Object} AppState
   * @property {AppConfig} config 运行配置
   * @property {HTMLElement[]} chapterNodes 章节节点列表
   * @property {Array<any>} quizAnswerCache 测验答案缓存
   */

  /**
   * 从后端加载配置
   * @param {<T = any>(cmd: string, args?: Record<string, any>) => Promise<T>} invoke
   * @param {AppConfig} config
   * @returns {Promise<AppConfig>}
   */
  const loadConfig = async (invoke, config) => {
    if (!invoke) {
      console.info("检测为无后端模式，使用默认配置");
      return config;
    }
    try {
      console.info("正在从后端加载配置...");
      const configResponse = await invoke("options");
      // 具体见后端 src-tauri\src\config\options.rs
      config = {
        ...config,
        hasBackend: true,
        muteVideo: configResponse.muteWebview,
        lockingSpeed: configResponse.speedLock,
        videoSpeedValue: configResponse.speedValue,
      };
      console.info("配置设置成功：", config);
    } catch (e) {
      console.error(e);
    }
    return config;
  };

  /**
   * @param {Document} document
   * @returns {HTMLElement[]}
   */
  const catchChapterNodes = (document) => {
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
   * @returns {Promise<"Video" | "PDF" | "Quiz" | "Other">}
   */
  const classifyTask = async (taskDoc) => {
    return "Other";
  };

  /**
   * @param {Document} chapterDoc
   */
  const handleTab = async (chapterDoc) => {
    if (!chapterDoc) {
      console.warn("无法获取章节主框架，请检查网络连接或页面加载状态");
      return;
    }
    /** @type {HTMLIFrameElement[]} */
    const taskIframes = await wait.elements(
      null,
      ".ans-attach-ct:has(.ans-job-icon) iframe",
      chapterDoc,
    );
    for (const taskIframe of taskIframes) {
      const taskDoc = await wait.until(() => getReadyIframeDoc(taskIframe));
      switch (await classifyTask(taskDoc)) {
        case "Video":
          break;
        case "PDF":
          break;
      }
    }
  };

  /**
   * @param {Document} doc
   * @param {HTMLElement} node
   */
  const handleChapter = async (doc, node) => {
    /** @type {HTMLElement | null} */
    const nameSpan = node.querySelector(DOM.courseTree.nameClass);
    console.info(`开始进入章节[${nameSpan?.getAttribute("title")}]`);
    const previewTabs = await wait.elements(
      () => nameSpan?.click(),
      DOM.chapter.tabClass,
      doc,
    );
    for (const tab of previewTabs) {
      console.info("等待章节主框架加载");
      const chapterDoc = await wait.iframeDoc(
        () => tab.click(),
        DOM.frames.chapterFrameId,
        doc,
      );
      await handleTab(chapterDoc);
    }
    console.info("本章节处理完毕");
  };

  const main = async () => {
    // 运行时状态与配置
    /** @type {AppState} */
    const state = {
      config: {
        hasBackend: false,
        muteVideo: true,
        lockingSpeed: false,
        videoSpeedValue: 2.0,
      },
      chapterNodes: [],
      quizAnswerCache: [],
    };
    state.config = await loadConfig(tauriInvoke, state.config);
    state.chapterNodes = catchChapterNodes(document);
    confirm("开始答题");
    for (const node of state.chapterNodes) {
      const status = chapterNodeStatus(node);
      if (status === "Interactive") {
        await handleChapter(document, node);
      }
    }
  };

  (async () => await main())();
})();
