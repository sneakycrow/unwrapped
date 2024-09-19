import ThemeToggle from "./ThemeToggle";

type FooterProps = {
  className?: string;
};

const Footer = (props: FooterProps) => {
  return (
    <footer className={props.className}>
      <ThemeToggle />
    </footer>
  );
};

export default Footer;
