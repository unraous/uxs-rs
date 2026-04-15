(function() {
    const OBSERVER_CONFIG = { childList: true, subtree: true, attributes: true };
    console.info('开始注入链接处理脚本...');

    const injectCss = () => {
        const style = document.createElement('style');
        style.textContent = `
            :root {
                background-color: #F6F8FA;
            }
        `;
        document.head.appendChild(style);
        console.info('默认背景 已修改');
    };

    const getIframeDoc = (iframe) => {
        try {
            const iframeDoc = iframe.contentDocument || iframe.contentWindow?.document;
            
            if (iframeDoc?.location.href !== 'about:blank' && iframeDoc?.body) {
                console.info('成功访问 iframe:', iframe);
                return iframeDoc;
            }

            console.warn('iframe 尚未加载完成，跳过:', iframe);
        } catch (e) {
            console.error('访问 iframe 失败:', e);
        }
        return null;
    };

    const getAllValidIframes = async (retryCount = 0, maxRetries = 4) => {
        const iframes = document.querySelectorAll('iframe');
        const validDocs = [];

        for (const iframe of iframes) {
            const doc = getIframeDoc(iframe);
            if (doc) validDocs.push(doc);
        }

        console.info(
            `第 ${retryCount + 1}/${maxRetries + 1} 次尝试：应获取 ${iframes.length} 个有效 iframe, 实际获取 ${validDocs.length} 个`
        );

        await new Promise(resolve => setTimeout(resolve, 500));
        if (iframes.length - validDocs.length > 0 && retryCount < maxRetries) {
            const moreIframes = await getAllValidIframes(retryCount + 1, maxRetries);
            validDocs.push(...moreIframes);
        }

        return validDocs;
    };

    const handleLinks = async () => {
        try {
            console.info('执行链接处理...');
            const docs = [document];
            
            const iframeDocs = await getAllValidIframes();
            docs.push(...iframeDocs);

            console.info(`找到 ${docs.length} 个文档进行处理`);

            for (const doc of docs) {
                try {
                    const targets = [...doc.querySelectorAll('a[href]')].filter(
                        a => a.getAttribute('target') === '_blank'
                    );
                    console.info(`处理文档 ${doc === document ? '主文档' : 'iframe'}，找到 ${targets.length} 个链接`);
                        
                    for (const target of targets) target.setAttribute('target', '_top');

                } catch (e) {
                    console.error('处理文档链接时出错:', e);
                }
            }
            console.info('链接处理完成');

        } catch (e) {
            console.error('执行链接处理时出错:', e);
        }
    };

    try {
        injectCss();
        handleLinks();
        const linkObserver = new MutationObserver((mutations) => {
            if (mutations.some(m => m.addedNodes.length > 0)) {
                console.info('检测到组件挂载，重新处理链接...');
                handleLinks();
            }
        });
        linkObserver.observe(document.body, OBSERVER_CONFIG);
        console.info('链接处理脚本注入完成');

    } catch (e) {
        console.error('注入链接处理脚本失败:', e);
    }
})();