import { configSchema, type Config } from './schema';

function loadConfig(): Config {
  const result = configSchema.safeParse(process.env);

  if (!result.success) {
    const errors = result.error.issues
      .map((issue) => `  • ${issue.path.join('.')}: ${issue.message}`)
      .join('\n');

    console.error(
      '\n' +
      '╔══════════════════════════════════════════════════╗\n' +
      '║        INVALID ENVIRONMENT CONFIGURATION         ║\n' +
      '╚══════════════════════════════════════════════════╝\n' +
      `${errors}\n`
    );

    process.exit(1);
  }

  // result.success is true here; data is guaranteed to be defined.
  return result.data as Config;
}

export const config: Config = loadConfig();
export type { Config };
