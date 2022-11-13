import { semver, zod } from "./deps.ts";

export const VersionUpdate = zod.object({
  from: zod.string().refine(semver.valid, "Valid semver").transform((data) =>
    semver.parse(data)!
  ),
  to: zod.string().refine(semver.valid, "Valid semver").transform((data) =>
    semver.parse(data)!
  ),
});
export type VersionUpdate = zod.infer<typeof VersionUpdate>;
export const StableVersion = zod.object({
  stable: zod.string().refine(semver.valid, "Valid semver").transform((data) =>
    semver.parse(data)!
  ),
});
export type StableVersion = zod.infer<typeof StableVersion>;
export const isStableVersion = (version: Version): version is StableVersion => {
  return "stable" in version;
};

export const Version = zod.union([VersionUpdate, StableVersion]);
export type Version = zod.infer<typeof Version>;

export const APIVersionInformation = zod.object({
  "core-linux32": zod.array(Version),
  "core-linux64": zod.array(Version),
  "core-linux_headless64": zod.array(Version),
  "core-mac": zod.array(Version),
  "core-win32": zod.array(Version),
  "core-win64": zod.array(Version),
});
export type APIVersionInformation = zod.infer<typeof APIVersionInformation>;
