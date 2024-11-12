import type { TransitionConfig } from 'svelte/transition';
import { cubicInOut } from 'svelte/easing';

export function grow(
  node: HTMLElement, 
  { 
    delay = 0, 
    duration = 400 
  }: { 
    delay?: number; 
    duration?: number; 

  } = {}
): TransitionConfig {
  // Get the natural width of the content
  const width = node.scrollWidth;
  
  // Set initial styles to prevent content wrapping during animation

  return {
    delay,
    duration,
    easing: cubicInOut,
    css: (t) => {
      return `
        max-width: ${t * width}px;
      `;
    },
    // Clean up styles after transition
    /*
    tick: (t) => {
      if (t === 1) {
        node.style.whiteSpace = '';
        node.style.overflow = '';
      }
    }*/
  };
}