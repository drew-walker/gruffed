import { loadNativeBinding } from "./lib/native-loader.js";

const native = loadNativeBinding();

export const analyzeGraph = native.analyzeGraph;
export const buildModuleGraph = native.buildModuleGraph;
export const freeGraph = native.freeGraph;
export const renderReport = native.renderReport;
