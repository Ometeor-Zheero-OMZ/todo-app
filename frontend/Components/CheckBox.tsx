import React from "react";
import ICON from "./SVG/ICON";

type CheckBoxProps = {
  toggleCompleted: (index: number) => void;
  index: number;
  todo: {
    title: string;
    completed: boolean;
  };
};

const CheckBox: React.FC<CheckBoxProps> = ({
  toggleCompleted,
  index,
  todo,
}) => {
  return (
    <label className="container">
      <input
        checked={todo.completed}
        type="checkbox"
        onChange={() => toggleCompleted(index)}
      />
      <ICON />
    </label>
  );
};

export default CheckBox;
