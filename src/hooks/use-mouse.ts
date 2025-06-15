import { ref } from "vue";

const isLeftMouseDown = ref(false);

function handleGlobalMouseDown(event: MouseEvent) {
  if (event.button === 0) {
    isLeftMouseDown.value = true;
  }
}

function handleGlobalMouseUp(event: MouseEvent) {
  if (event.button === 0) {
    isLeftMouseDown.value = false;
  }
}

document.addEventListener("mousedown", handleGlobalMouseDown);
document.addEventListener("mouseup", handleGlobalMouseUp);

export function useMouse() {
  return {
    isLeftMouseDown,
  };
}
