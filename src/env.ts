import { envalid } from "./deps.ts";
import { dotenv } from "./deps.ts";
const env = dotenv.config({ path: "./.env" });

const ENV = envalid.cleanEnv(env, {
  username: envalid.str({ desc: "Username for Factorio" }),
  token: envalid.str({ desc: "Token for Factorio" }),
});
export default ENV;
