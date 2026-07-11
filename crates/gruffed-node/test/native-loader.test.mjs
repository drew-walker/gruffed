import { describe, expect, test } from "vitest";

import {
  candidateTriples,
  isMissingOptionalPackage,
  loadNativeBinding,
} from "../lib/native-loader.js";

describe("native loader", () => {
  test("maps supported platforms to package triples", () => {
    expect(candidateTriples("darwin", "arm64")).toEqual(["darwin-arm64"]);
    expect(candidateTriples("darwin", "x64")).toEqual(["darwin-x64"]);
    expect(candidateTriples("win32", "arm64")).toEqual(["win32-arm64-msvc"]);
    expect(candidateTriples("win32", "ia32")).toEqual(["win32-ia32-msvc"]);
    expect(candidateTriples("win32", "x64")).toEqual(["win32-x64-msvc"]);
    expect(candidateTriples("linux", "arm64")).toEqual(["linux-arm64-gnu", "linux-arm64-musl"]);
    expect(candidateTriples("linux", "arm")).toEqual([
      "linux-arm-gnueabihf",
      "linux-arm-musleabihf",
    ]);
    expect(candidateTriples("linux", "x64")).toEqual(["linux-x64-gnu", "linux-x64-musl"]);
    expect(candidateTriples("freebsd", "x64")).toEqual(["freebsd-x64"]);
  });

  test("recognizes missing optional package errors", () => {
    expect(
      isMissingOptionalPackage(
        Object.assign(new Error("Cannot find module '@gruffed/node-linux-x64-gnu'"), {
          code: "MODULE_NOT_FOUND",
        }),
        "@gruffed/node-linux-x64-gnu",
      ),
    ).toBe(true);
    expect(isMissingOptionalPackage(new Error("boom"), "@gruffed/node-linux-x64-gnu")).toBe(false);
  });

  test("loads a matching local native binding first", () => {
    const loaded = { analyzeGraph() {} };
    const result = loadNativeBinding({
      baseDir: "/native",
      platform: "linux",
      arch: "x64",
      existsSync: (filePath) => filePath.endsWith("gruffed-node.linux-x64-gnu.node"),
      requireFn: () => loaded,
    });

    expect(result).toBe(loaded);
  });

  test("loads a generic local native binding fallback", () => {
    const loaded = { buildModuleGraph() {} };
    const result = loadNativeBinding({
      baseDir: "/native",
      platform: "linux",
      arch: "x64",
      existsSync: (filePath) => filePath.endsWith("gruffed-node.node"),
      requireFn: () => loaded,
    });

    expect(result).toBe(loaded);
  });

  test("loads the first installed optional native package", () => {
    const loaded = { freeGraph() {} };
    const result = loadNativeBinding({
      platform: "linux",
      arch: "arm64",
      existsSync: () => false,
      requireFn: (packageName) => {
        if (packageName === "@gruffed/node-linux-arm64-musl") {
          return loaded;
        }

        throw Object.assign(new Error(`Cannot find module '${packageName}'`), {
          code: "MODULE_NOT_FOUND",
        });
      },
    });

    expect(result).toBe(loaded);
  });

  test("rethrows unexpected optional package load errors", () => {
    const error = new Error("native module crashed while loading");

    expect(() =>
      loadNativeBinding({
        platform: "linux",
        arch: "x64",
        existsSync: () => false,
        requireFn: () => {
          throw error;
        },
      }),
    ).toThrow(error);
  });

  test("reports searched local files and optional packages", () => {
    expect(() =>
      loadNativeBinding({
        platform: "freebsd",
        arch: "x64",
        existsSync: () => false,
        requireFn: (packageName) => {
          throw Object.assign(new Error(`Cannot find module '${packageName}'`), {
            code: "MODULE_NOT_FOUND",
          });
        },
      }),
    ).toThrow(
      "Could not find a native gruffed binding for freebsd/x64. Looked for local files: gruffed-node.freebsd-x64.node, gruffed-node.node; optional packages: @gruffed/node-freebsd-x64",
    );
  });
});
