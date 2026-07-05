export interface BuildResultJs {
  graphHandle: number;
  warnings: JsBuildWarning[];
  stats: JsBuildStats;
}

export interface JsBuildStats {
  filesScanned: number;
  buildTimeMs: number;
}

export interface JsBuildWarning {
  kind: string;
  source: string;
  specifier?: string;
  line?: number;
  error?: string;
}

export interface JsReport {
  findings: JsFinding[];
  stats: JsGraphStats;
}

export interface JsFinding {
  ruleId: string;
  severity: string;
  message: string;
  file: string;
  line?: number;
}

export interface JsGraphStats {
  nodeCount: number;
  edgeCount: number;
}

export function buildModuleGraph(root: string, configJson?: string): BuildResultJs;

export function analyzeGraph(
  graphHandle: number,
  warnings: JsBuildWarning[],
  rulesJson: string,
): JsReport;

export function renderReport(
  report: JsReport,
  graphHandle: number,
  useColor: boolean,
): string;

export function freeGraph(graphHandle: number): void;
