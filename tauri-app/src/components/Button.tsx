import { ReactNode, MouseEvent } from "react";

type ButtonVariant = "primary" | "outline";
type ButtonProps = {
  onClick: (event: MouseEvent<HTMLButtonElement>) => void;
  variant?: ButtonVariant;
  children: ReactNode;
};

const sharedClasses =
  "w-full max-w-[200px] rounded py-2 px-4 text-center hover:cursor-pointer transition-colors transition-opacity";
const variantClasses = {
  primary: "bg-primary border-2 text-black border-primary hover:text-white",
  outline:
    "bg-transparent text-primary border-2 border-primary hover:border-black hover:text-black dark:hover:text-white dark:hover:border-white",
};
const Button = (props: ButtonProps) => {
  const { variant = "primary" } = props;
  const buttonClass = `${sharedClasses} ${variantClasses[variant]}`;
  return (
    <button className={buttonClass} onClick={props.onClick}>
      {props.children}
    </button>
  );
};

export default Button;
