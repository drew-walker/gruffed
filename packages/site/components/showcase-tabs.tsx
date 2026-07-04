"use client";

import * as Tabs from "@radix-ui/react-tabs";
import { TerminalOutput } from "./terminal-output";
import styles from "./showcase-tabs.module.css";

export type ShowcaseSample = {
  id: string;
  label: string;
  output: string;
};

export function ShowcaseTabs({ samples }: Readonly<{ samples: ShowcaseSample[] }>) {
  return (
    <Tabs.Root className={styles.root} defaultValue={samples[0]?.id}>
      <Tabs.List className={styles.list} aria-label="Sample findings">
        {samples.map((sample) => (
          <Tabs.Trigger className={styles.trigger} value={sample.id} key={sample.id}>
            {sample.label}
          </Tabs.Trigger>
        ))}
      </Tabs.List>
      {samples.map((sample) => (
        <Tabs.Content className={styles.content} value={sample.id} key={sample.id}>
          <TerminalOutput label={sample.label} output={sample.output} />
        </Tabs.Content>
      ))}
    </Tabs.Root>
  );
}
