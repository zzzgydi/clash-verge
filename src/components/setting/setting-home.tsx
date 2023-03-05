import useSWR from "swr";
import { useRef } from "react";
import { useTranslation } from "react-i18next";
import { BaseEmpty, BasePage } from "@/components/base";
import { version } from "@root/package.json";
import {
  Box,
  Switch,
  Button,
  IconButton,
  MenuItem,
  Paper,
  Select,
  TextField,
  Typography,
  alpha
} from "@mui/material";

import { ArrowForward, PrivacyTipRounded, Settings } from "@mui/icons-material";
import { checkService } from "@/services/cmds";
import { useVerge } from "@/hooks/use-verge";

import { DialogRef } from "@/components/base";
import { SettingList, SettingItem } from "./mods/setting-comp";
import { GuardState } from "./mods/guard-state";
import { ServiceViewer } from "./mods/service-viewer";
import { SysproxyViewer } from "./mods/sysproxy-viewer";
import getSystem from "@/utils/get-system";

interface Props {
  onError?: (err: Error) => void;
}

const isWIN = getSystem() === "windows";

const Home = ({ onError }: Props) => {
  const { t } = useTranslation();

  const { verge, mutateVerge, patchVerge } = useVerge();
  const { language } = verge ?? {};
  // service mode
  const { data: serviceStatus } = useSWR(
    isWIN ? "checkService" : null,
    checkService,
    {
      revalidateIfStale: false,
      shouldRetryOnError: false,
      focusThrottleInterval: 36e5, // 1 hour
    }
  );

  const serviceRef = useRef<DialogRef>(null);
  const sysproxyRef = useRef<DialogRef>(null);

  const {
    enable_tun_mode,
    enable_auto_launch,
    enable_service_mode,
    enable_silent_start,
    enable_system_proxy,
  } = verge ?? {};

  const onSwitchFormat = (_e: any, value: boolean) => value;
  const onChangeData = (patch: Partial<IVergeConfig>) => {
    mutateVerge({ ...verge, ...patch }, false);
  };
  const is_admin = false
  return (
    <SettingList title={t("Quick Actions")}>
      

      <SettingItem label={t("Enable")}>
        <GuardState
          value={enable_tun_mode ?? enable_system_proxy ?? false}
          valueProps="checked"
          onCatch={onError}
          onFormat={onSwitchFormat}
          onChange={(e) => {
            if (is_admin) {
              onChangeData({ enable_tun_mode: e })
            } else {
              onChangeData({ enable_system_proxy: e })
            }

          }}
          onGuard={(e) => {            
            if (is_admin) {
              return patchVerge({ enable_tun_mode: e });
            } else {
              return patchVerge({ enable_system_proxy: e })
            }            
          }}
        >
          <Switch edge="end" />
        </GuardState>
      </SettingItem>
      <SettingItem label={t("Language")}>
        <GuardState
          value={language ?? "en"}
          onCatch={onError}
          onFormat={(e: any) => e.target.value}
          onChange={(e) => onChangeData({ language: e })}
          onGuard={(e) => patchVerge({ language: e })}
        >
          <Select size="small" sx={{ width: 100, "> div": { py: "7.5px" } }}>
            <MenuItem value="fa">فارسی</MenuItem>
            <MenuItem value="en">English</MenuItem>
            <MenuItem value="zh">中文</MenuItem>
          </Select>
        </GuardState>
      </SettingItem>

      <SettingItem label={t("Verge Version")}>
        <Typography sx={{ py: "7px", pr: 1 }}>v{version}</Typography>
      </SettingItem>
      {!is_admin && (
        <Box
        sx={({ palette }) => ({
          width: "100%",
          height: "100%",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          color: alpha(palette.text.secondary, 0.75),
        })}
      >

        <Typography sx={{ fontSize: "1.25em" }}>{t("Please run in admin mode for proxy whole system")}</Typography>
        
      </Box>
  
      )}
      <Box
        sx={({ palette }) => ({
          width: "100%",
          height: "100%",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          color: alpha(palette.text.secondary, 0.75),
        })}
      >

        <Typography sx={{ fontSize: "1.25em" }}>{t("Hiddify, For a free internet.")}</Typography>
        
      </Box>
      

    </SettingList>
  );
};

export default Home;
