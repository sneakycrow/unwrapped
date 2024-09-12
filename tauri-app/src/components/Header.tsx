import Logo from "./Logo";

type HeaderProps = {
  title?: string;
};

const Header = (props: HeaderProps) => {
  const { title } = props;
  return (
    <header className="grid grid-cols-4 h-full max-h-[100px] justify-start items-center p-4">
      <Logo size={75} />
      {title && (
        <h1 className="text-4xl font-extrabold col-span-3 text-right hover:text-primary">
          {title}
        </h1>
      )}
    </header>
  );
};

export default Header;
