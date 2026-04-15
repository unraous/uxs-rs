(function() {
    const mask = document.createElement('div');
    mask.id = 'fade-in-mask';
    mask.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: #000000;
        opacity: 0;
        z-index: 999999;
        pointer-events: none;
        transition: opacity 500ms ease-in-out;
    `;

    document.documentElement.appendChild(mask);

    // 确保样式已应用，下一帧启动过渡
    requestAnimationFrame(() => {
        mask.style.opacity = '1';
    });
})();