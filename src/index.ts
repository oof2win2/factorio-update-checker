import ENV from "./env.ts";
import {
  APIVersionInformation,
  isStableVersion,
  StableVersion,
  Version,
  VersionUpdate,
} from "./typings.ts";
import { semver } from "./deps.ts";

async function get_available_versions(): Promise<APIVersionInformation> {
  const params = new URLSearchParams();
  params.append("username", ENV.username);
  params.append("token", ENV.token);
  const versions = await fetch(
    `https://updater.factorio.com/get-available-versions?${params.toString()}`,
  )
    .then((response) => response.json());
  return APIVersionInformation.parse(versions);
}

function get_latest_stable(versions: Version[]): semver.SemVer {
  let latest = semver.parse("0.0.0")!;
  for (const version of versions) {
    if ("stable" in version) {
      if (version.stable.compare(latest) > 0) {
        latest = version.stable;
      }
    }
  }
  return latest;
}

const versions = await get_available_versions();
console.log(get_latest_stable(versions["core-win64"]));
