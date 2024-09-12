import LogoGreenSVG from "../assets/logo_green.svg";

type LogoProps = {
  size?: number;
};

const Logo = (props: LogoProps) => {
  const { size = 100 } = props;

  return <img src={LogoGreenSVG} width={size} />;
};

export default Logo;
