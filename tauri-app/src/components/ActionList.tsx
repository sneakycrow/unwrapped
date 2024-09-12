import Button from "./Button";

type ActionListProps = {};

const ActionList = (props: ActionListProps) => {
  const handleClick = () => {
    // console.log("Button clicked");
  };
  return (
    <section className="flex flex-col space-y-4 items-center justify-center w-full h-full">
      <Button onClick={handleClick}>Recent Tracks</Button>
      <Button onClick={handleClick} variant="outline">
        Top Artists
      </Button>
    </section>
  );
};

export default ActionList;
