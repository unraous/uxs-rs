<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import * as THREE from 'three';
import { EffectComposer } from 'three/examples/jsm/postprocessing/EffectComposer.js';
import { RenderPass } from 'three/examples/jsm/postprocessing/RenderPass.js';
import { GlitchPass } from 'three/examples/jsm/postprocessing/GlitchPass.js';

const canvasContainer = ref<HTMLDivElement>();
class GlitchMagic {
  scene: THREE.Scene | null = null;
  camera: THREE.PerspectiveCamera | null = null;
  renderer: THREE.WebGLRenderer | null = null;
  composer: EffectComposer | null = null;
  glitch: GlitchPass | null = null;
  uniforms: any = {};
  speed: number = 0.02;
  intensity: number = 0.5;
  mousePosition: boolean = true;
  wildGlitch: boolean = false;
  color: number[] = [137, 188, 222];
  offset: { x: number; y: number } = { x: 3, y: 3 };
  mouse: { x: number; y: number } = { x: 0, y: 0 };
  innerWidth: number = 0;
  innerHeight: number = 0;
  container: HTMLElement;
  animationId: number | null = null;
  resizeObserver: ResizeObserver | null = null;
  glitchTime: number = 0;
  flickerStrength: number = 0;

  constructor(container: HTMLElement) {
    this.container = container;
    // 获取容器的实际尺寸
    const rect = container.getBoundingClientRect();
    this.innerWidth = rect.width;
    this.innerHeight = rect.height;
  }

  begin() {
    // Scene setup
    this.scene = new THREE.Scene();
    
    this.camera = new THREE.PerspectiveCamera(
      50,
      this.innerWidth / this.innerHeight,
      0.5,
      1000
    );
    this.camera.position.set(0, 0, 1);
    this.scene.add(this.camera);

    // Renderer setup
    this.renderer = new THREE.WebGLRenderer({
      antialias: true,
      preserveDrawingBuffer: true,
    });
    this.renderer.setSize(this.innerWidth, this.innerHeight);
    this.renderer.setPixelRatio(window.devicePixelRatio);
    this.renderer.setClearColor(0x000000);
    this.renderer.clear();
    this.container.appendChild(this.renderer.domElement);
    
    this.renderer.domElement.addEventListener(
      'mousemove',
      this.onMouseMove.bind(this),
      false
    );

    // Composer setup
    this.composer = new EffectComposer(this.renderer);
    this.composer.addPass(new RenderPass(this.scene, this.camera));

    this.glitch = new GlitchPass(64);
    this.glitch.renderToScreen = true;
    this.glitch.goWild = this.wildGlitch;
    this.composer.addPass(this.glitch);

    // 监听容器尺寸变化
    this.resizeObserver = new ResizeObserver(() => {
      this.resize();
    });
    this.resizeObserver.observe(this.container);

    this.createMesh();
    this.animate();
  }

  rgbToPercentage(arr: number[]): number[] {
    return arr.map((value) => value / 255);
  }

  onMouseMove(event: MouseEvent) {
    event.preventDefault();

    this.mouse.x = this.easeOutQuad(
      (event.clientX - this.renderer!.domElement.width / 2) /
        this.renderer!.domElement.width
    );
    this.mouse.y = this.easeOutQuad(
      -(
        (event.clientY - this.renderer!.domElement.height / 2) /
          this.renderer!.domElement.height
      )
    );
  }

  easeOutQuad(t: number): number {
    return t * (2 - t);
  }

  createMesh() {
    this.uniforms = {
      resolution: { value: new THREE.Vector2(this.innerWidth, this.innerHeight) },
      time: { value: 0 },
      flickerStrength: { value: 0 }
    };

    const material = new THREE.ShaderMaterial({
      vertexShader: `
        void main() {
            gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader: `
        precision highp float;
        
        uniform vec2 resolution;
        uniform float time;
        uniform float flickerStrength;
        
        float rand(vec2 co) {
            return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453);
        }
        
        void main() {
            vec2 uv = gl_FragCoord.xy / resolution;
            
            // 瞬间切换：flickerStrength 要么是 1，要么是 0
            if (flickerStrength < 0.5) {
                gl_FragColor = vec4(0.0); // 完全透明
                return;
            }
            
            // 闪烁时的强烈效果 - 不做平缓混合
            vec3 glitchColor = vec3(
                sin(uv.x * 3.14159 + time) * 0.5 + 0.5,
                cos(uv.y * 3.14159 + time) * 0.5 + 0.5,
                rand(uv * 10.0 + time)
            );
            
            gl_FragColor = vec4(glitchColor, 1.0); // 完全不透明
        }
            `,
      uniforms: this.uniforms,
      transparent: true,
      blending: THREE.AdditiveBlending
    });
    
    const geometry = new THREE.PlaneGeometry(
      this.innerWidth,
      this.innerHeight,
      1
    );
    const mesh = new THREE.Mesh(geometry, material);
    this.scene!.add(mesh);
  }

  animate = () => {
    this.glitchTime += 0.016; // 大约 60fps
    this.uniforms.time.value = this.glitchTime;
    this.uniforms.flickerStrength.value = this.flickerStrength;
    
    // 统一的闪烁周期
    const flickerCycle = 5; // 1000ms 一个周期
    const cycleProgress = (this.glitchTime % flickerCycle) / flickerCycle;
    
    // 瞬间切换：闪烁 0.2s，休息 0.8s
    const strength = cycleProgress < 0.2 ? 1 : 0;
    
    this.flickerStrength = strength;
    this.uniforms.time.value = this.glitchTime;
    this.uniforms.flickerStrength.value = strength;
    
    // 同步 GlitchPass
    this.glitch!.goWild = strength > 0.5;
    
    this.render();
  };

  resize = () => {
    const rect = this.container.getBoundingClientRect();
    this.innerWidth = rect.width;
    this.innerHeight = rect.height;

    this.camera!.aspect = this.innerWidth / this.innerHeight;
    this.camera!.updateProjectionMatrix();

    this.uniforms.resolution.value.x = this.innerWidth;
    this.uniforms.resolution.value.y = this.innerHeight;

    this.composer!.setSize(this.innerWidth, this.innerHeight);
    this.renderer!.setSize(this.innerWidth, this.innerHeight);
  };

  render() {
    this.animationId = globalThis.requestAnimationFrame(this.animate);
    this.composer!.render();
  }

  destroy() {
    if (this.animationId) {
      cancelAnimationFrame(this.animationId);
    }
    if (this.resizeObserver) {
      this.resizeObserver.disconnect();
    }
    if (this.renderer && this.container.contains(this.renderer.domElement)) {
      this.renderer.domElement.remove();
      this.renderer.dispose();
    }
  }
}

let glitch: GlitchMagic | null = null;

onMounted(() => {
  if (!canvasContainer.value) return;
  glitch = new GlitchMagic(canvasContainer.value);
  glitch.begin();
});

onUnmounted(() => {
  if (glitch) {
    glitch.destroy();
  }
});
</script>

<template>
  <div ref="canvasContainer" class="glitch-container"></div>
</template>

<style scoped>
.glitch-container {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background-color: #000;
}
</style>