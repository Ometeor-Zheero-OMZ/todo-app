"use client";
import { useState, useEffect } from "react";
import { MdDelete, MdEdit, MdConfirmationNumber } from "react-icons/md";
import axios from "axios";
import { format } from "date-fns";
import CheckBox from "../Components/CheckBox";

export default function Home() {
  type Todo = {
    id: number;
    title: string;
    completed: boolean;
    created_at: string;
  };

  const [todos, setTodos] = useState<Todo[]>([]);
  const [todosCopy, setTodosCopy] = useState<Todo[]>(todos);
  const [todoInput, setTodoInput] = useState<string>("");
  const [editFlag, setEditFlag] = useState<number | boolean>(false);
  const [searchInput, setSearchInput] = useState<string>("");

  const [count, setCount] = useState<number>(0);
  const [search, setSearch] = useState<string>("");
  const [searchItem, setSearchItem] = useState(search);

  const [validationMessage, setValidationMessage] = useState<string>("");

  useEffect(() => {
    fetchTodos();
  }, [count]);

  const editTodo = (index: number) => {
    setTodoInput(todos[index].title);
    setEditFlag(index); // Set editFlag to the index of the todo being edited
  };

  const fetchTodos = async () => {
    try {
      const response = await axios.get("http://127.0.0.1:8080/todos");
      setTodos(response.data);
      setTodosCopy(response.data);
    } catch (error) {
      console.error(error);
    }
  };

  const addTodo = async () => {
    try {
      if (todoInput.trim() === "") {
        setValidationMessage("タスクのタイトルを入力してください。");
        return; // Early return if input is invalid
      }

      if (typeof editFlag === "number") {
        // Editing an existing todo
        const todoToUpdate = {
          ...todos[editFlag],
          title: todoInput,
        };
        const response = await axios.put(
          `http://127.0.0.1:8080/todos/${todoToUpdate.id}`,
          todoToUpdate
        );
        const updatedTodos = todos.map((todo, index) =>
          index === editFlag ? response.data : todo
        );
        setTodos(updatedTodos);
        setTodosCopy(updatedTodos); // Ensure todosCopy is updated as well
        setTodoInput(""); // Clear input after update
        setEditFlag(false); // Reset editFlag
      } else {
        // Adding a new todo
        const response = await axios.post("http://127.0.0.1:8080/todos", {
          title: todoInput,
          completed: false,
          created_at: new Date().toISOString(), // Set created_at to current date
        });
        console.log("New Todo Response:", response.data); // Log the response
        setTodos([...todos, response.data]); // Append new todo
        setTodosCopy([...todosCopy, response.data]); // Append new todo
        setTodoInput(""); // Clear input after adding
      }

      setValidationMessage(""); // Clear validation message
    } catch (error) {
      console.error(error);
    }
  };

  const deleteTodo = async (id: number) => {
    try {
      await axios.delete(`http://127.0.0.1:8080/todos/${id}`);
      setTodos(todos.filter((todo) => todo.id !== id));
      setValidationMessage("");
      setEditFlag(false);
    } catch (error) {
      console.error(error);
    }
  };

  const toggleCompleted = async (index: number) => {
    try {
      const filteredTodo = todos[index];
      const todoToUpdate = {
        ...filteredTodo,
        completed: !todos[index].completed,
      };
      const response = await axios.put(
        `http://127.0.0.1:8080/todos/${todoToUpdate.id}`,
        todoToUpdate
      );
      const updatedTodos = todos.map((todo) =>
        todo.id === todoToUpdate.id ? response.data : todo
      );
      updatedTodos[index] = response.data;

      setTodos(updatedTodos);
      setTodosCopy(updatedTodos);
      setCount(count + 1);
    } catch (error) {
      console.error(error);
    }
  };

  const searchTodo = () => {
    const results = todos.filter((todo) =>
      todo.title.toLowerCase().includes(searchInput.toLowerCase())
    );
  };

  // change
  const formatDate = (dateString: string) => {
    if (!dateString) {
      return "No date"; // Handle missing date case
    }

    const date = new Date(dateString);
    return isNaN(date.getTime())
      ? "Invalid date"
      : format(date, "yyyy-MM-dd HH:mm:ss");
  };

  const renderTodos = (todosToRender: Todo[]) => {
    return todosToRender.map((todo, index) => (
      <li key={todo.id} className="li">
        {" "}
        {/* Use todo.id for unique key */}
        <CheckBox toggleCompleted={toggleCompleted} index={index} todo={todo} />
        <label className="form-check-label"></label>
        <span className="todo-text">
          {`${todo.title} ${formatDate(todo.created_at)}`}
        </span>
        <span className="span-button" onClick={() => deleteTodo(todo.id)}>
          <MdDelete />
        </span>
        <span className="span-button" onClick={() => editTodo(index)}>
          <MdEdit />
        </span>
      </li>
    ));
  };

  // filter
  const onHandleSearch = (value: string) => {
    const filteredTodo = todosCopy.filter(
      ({ title }) => title && title.toLowerCase().includes(value.toLowerCase())
    );
    setTodos(filteredTodo);
  };

  const onClearSearch = () => {
    if (todos.length && todosCopy.length) {
      setTodos(todosCopy);
    }
  };

  useEffect(() => {
    const timer = setTimeout(() => setSearch(searchItem), 100);
    return () => clearTimeout(timer);
  }, [searchItem]);

  useEffect(() => {
    if (search) {
      onHandleSearch(search);
    } else {
      onClearSearch();
    }
  }, [search]);

  return (
    <div className="main-body">
      {validationMessage && (
        <div className="message-area">
          <div className="validation-message">{validationMessage}</div>
        </div>
      )}
      <div className="todo-app">
        <div className="input-section">
          <input
            type="text"
            id="todoInput"
            placeholder="Add item.."
            value={todoInput}
            onChange={(e) => setTodoInput(e.target.value)}
          />
          <button onClick={() => addTodo()} className="add">
            {editFlag === false || todos.length === 0 ? "Add" : "Update"}
          </button>
          <input
            type="text"
            id="searchInput"
            placeholder="Search item..."
            value={searchItem}
            onChange={(e) => setSearchItem(e.target.value)}
          />
          <button onClick={() => {}}>Search</button>
        </div>

        <div className="todos">
          <ul className="todo-list">{renderTodos(todos)}</ul>
          {todos.length === 0 && (
            <div className="todo-area">
              <h1 className="not-found">NOT FOUND</h1>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
