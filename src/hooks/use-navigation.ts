import { ref } from "vue";

export type View = "setup" | "recorder";

const currentView = ref<View>("setup");

function navigateTo(view: View) {
  currentView.value = view;
}

export function useNavigation() {
  return {
    currentView,
    navigateTo,
  };
}
