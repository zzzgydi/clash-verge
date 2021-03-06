import { useTranslation } from "react-i18next";
import { Button, ButtonGroup } from "@mui/material";
import { CmdType } from "../../services/types";

type ThemeValue = CmdType.VergeConfig["theme_mode"];

interface Props {
  value?: ThemeValue;
  onChange?: (value: ThemeValue) => void;
}

const ThemeModeSwitch = (props: Props) => {
  const { value, onChange } = props;
  const { t } = useTranslation();

  const modes = ["light", "dark", "system"] as const;

  return (
    <ButtonGroup size="small">
      {modes.map((mode) => (
        <Button
          key={mode}
          variant={mode === value ? "contained" : "outlined"}
          onClick={() => onChange?.(mode)}
          sx={{ textTransform: "capitalize" }}
        >
          {t(`theme.${mode}`)}
        </Button>
      ))}
    </ButtonGroup>
  );
};

export default ThemeModeSwitch;
