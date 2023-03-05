import LogsPage from "./logs";
import ProxiesPage from "./proxies";
import HomePage from "./home";
import ProfilesPage from "./profiles";
import SettingsPage from "./settings";
import ConnectionsPage from "./connections";
import RulesPage from "./rules";

export const routers = [
  {
    label: "Label-Home",
    link: "/",
    ele: HomePage,
  },
  {
    label: "Label-Proxies",
    link: "/proxy",
    ele: ProxiesPage,
  },
  {
    label: "Label-Profiles",
    link: "/profile",
    ele: ProfilesPage,
  },
  {
    label: "Label-Settings",
    link: "/settings",
    ele: SettingsPage,
  },
  {
    label: "Label-Connections",
    link: "/connections",
    ele: ConnectionsPage,
  },
  {
    label: "Label-Rules",
    link: "/rules",
    ele: RulesPage,
  },
  {
    label: "Label-Logs",
    link: "/logs",
    ele: LogsPage,
  },
];
