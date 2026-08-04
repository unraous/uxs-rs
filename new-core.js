// @ts-check
(function () {
  "use strict";
  const console = globalThis.console;
  const document = globalThis.document;

  /**DOM 选择器字段 */
  const DOM_CONFIG = Object.freeze({
    courseTree: {
      containerId: "coursetree",
      nodeClass: "div.posCatalog_select",
      titleClass: "span.posCatalog_title",
      activeClass: "posCatalog_active",
      nameClass: "span.posCatalog_name",
      unfinishedClass: ".orangeNew",
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

    iframe: {
      loadingUrl: "about:blank",
      nextBtnId: "prevNextFocusNext",
      outerId: "iframe",
      innerCourseId: "iframe.ans-attach-online",
      innerCourseClass: "ans-attach-online",
      mainContentClass: ".content",
    },
  });

  /**
   * tauri backend command function
   *  @type {<T = any>(cmd: string, args?: Record<string, any>) => Promise<T>} */
  const tauriInvoke = /** @type {any} */ (globalThis).__TAURI_INTERNALS__
    ?.invoke;

  // 运行时状态与配置
  const state = {
    config: {
      hasBackend: false,
      muteVideo: true,
      lockingSpeed: false,
      videoSpeedValue: 2.0,
      maxTryCount: 50,
    },
    /** @type {HTMLElement[]} */
    chapterNodes: [],
    quizAnswerCache: [],
  };

  /** @param {number} ms */
  const sleep = (ms) =>
    new Promise((resolve) => globalThis.setTimeout(resolve, ms));

  /**
   * 从后端加载配置
   */
  const getConfig = async () => {
    if (!tauriInvoke) {
      console.info("检测为无后端模式，使用默认配置");
      return;
    }
    try {
      console.info("正在从后端加载配置...");
      const configResponse = await tauriInvoke("options");
      // 具体见后端 src-tauri\src\config\options.rs
      state.config = {
        ...state.config,
        hasBackend: true,
        muteVideo: configResponse.muteWebview,
        lockingSpeed: configResponse.speedLock,
        videoSpeedValue: configResponse.speedValue,
      };
      console.info("配置设置成功：", state.config);
    } catch (e) {
      console.error(e);
    }
  };

  const catchChapterNodes = () => {
    state.chapterNodes = Array.from(
      document
        .getElementById(DOM_CONFIG.courseTree.containerId)
        ?.querySelectorAll(DOM_CONFIG.courseTree.nodeClass) ?? [],
    );
    if (state.chapterNodes.length > 0) {
      console.info("获取课程列表成功：", state.chapterNodes);
    } else {
      console.error("获取课程列表失败");
    }
  };

  /**
   * 获取课程章节状态
   * @param {HTMLElement} node
   * @returns {'Blocking' | 'Interactive' | 'Finished' | 'Title' | 'Unknown'}
   */
  const chapterNodeStatus = (node) => {
    /** @type {HTMLElement | null} */
    const nameSpan = node.querySelector(DOM_CONFIG.courseTree.nameClass);
    if (!nameSpan) {
      return node.querySelector(DOM_CONFIG.courseTree.titleClass)
        ? "Title"
        : "Unknown";
    }
    if (nameSpan.onclick == null) {
      return "Blocking";
    }
    return node.querySelector(DOM_CONFIG.courseTree.unfinishedClass)
      ? "Interactive"
      : "Finished";
  };

  /** @param {HTMLElement} node */
  const handleChapter = async (node) => {
    /** @type {HTMLElement | null} */
    const nameSpan = node.querySelector(DOM_CONFIG.courseTree.nameClass);
    console.info(`开始处理章节[${nameSpan?.getAttribute("title")}]`);
    nameSpan?.click();
  };

  const main = async () => {
    await getConfig();
    catchChapterNodes();

    for (const node of state.chapterNodes) {
      const status = chapterNodeStatus(node);
      if (status === "Interactive") {
        await handleChapter(node);
        await sleep(2000);
      }
    }
  };

  (async () => await main())();
})();
