import { Paper } from "@mui/material";
import { useTranslation } from "react-i18next";
import { BasePage, Notice } from "@/components/base";
import Home from "@/components/setting/setting-home";

const HomePage = () => {
  const { t } = useTranslation();

  const onError = (err: any) => {
    Notice.error(err?.message || err.toString());
  };

  return (
    <BasePage title={t("Home")}>
      <Paper sx={{ borderRadius: 1, boxShadow: 2, mb: 3 }}>
        <Home onError={onError} />
      </Paper>
      

      
    </BasePage>
  );
};

export default HomePage;
