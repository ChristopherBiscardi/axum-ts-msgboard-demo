import { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";

function App() {
  const [messages, setMessages] = useState([]);
  useEffect(() => {
    fetch("http://localhost:3001/message")
      .then((v) => v.json())
      .then((response) => {
        setMessages(response.data);
      });

    // This isn't a production-ready example,
    // so just poll every second to get "live updates"
    // because that demos better
    setInterval(function () {
      fetch("http://localhost:3001/message")
        .then((v) => v.json())
        .then((response) => {
          setMessages(response.data);
        });
    }, 1000);
  }, []);

  return (
    <div className="mx-auto px-4 sm:px-6 lg:px-8">
      <div className="max-w-7xl mx-auto sm:px-6 lg:px-8">
        <div className="md:flex md:items-center md:justify-between">
          <div className="flex-1 min-w-0">
            <h2 className="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate mt-12">
              The only thread on this message board
            </h2>
          </div>
        </div>
        <Messages messages={messages} />
        <form
          className="sm:grid sm:grid-cols-3 sm:gap-4 sm:items-start sm:border-t sm:border-gray-200 sm:pt-5"
          onSubmit={(e) => {
            e.preventDefault();
            const data = Object.fromEntries(
              new FormData(e.target as any).entries()
            );
            console.log(data);
            fetch("http://localhost:3001/message", {
              method: "POST",
              // mode: 'cors', // no-cors, *cors, same-origin
              headers: {
                "Content-Type": "application/json",
              },

              body: JSON.stringify(data),
            })
              .then((response) => response.json())
              .then((response) => {
                console.log(response);
              });
          }}
        >
          <label
            htmlFor="message"
            className="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
          >
            Add a Message
          </label>
          <div className="mt-1 sm:mt-0 sm:col-span-2 max-w-lg">
            <textarea
              id="message"
              name="message"
              rows={3}
              className="shadow-sm block w-full focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm border border-gray-300 rounded-md p-4"
              defaultValue={""}
            />
            <div className="flex justify-between mt-2">
              <p className="text-sm text-gray-500">Join the Conversation!</p>
              <button
                type="submit"
                className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="-ml-1 mr-2 h-5 w-5"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path
                    fillRule="evenodd"
                    d="M7.707 3.293a1 1 0 010 1.414L5.414 7H11a7 7 0 017 7v2a1 1 0 11-2 0v-2a5 5 0 00-5-5H5.414l2.293 2.293a1 1 0 11-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                    clipRule="evenodd"
                  />
                </svg>
                Reply
              </button>
            </div>
          </div>
        </form>
      </div>
    </div>
  );
}

export default App;

interface MessageProps {
  messages: Message[];
}
interface Message {
  content: string;
}

function Messages({ messages }: MessageProps) {
  if (messages.length === 0) {
    return <div>no messages yet. Be the first!</div>;
  }
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <div className="max-w-3xl mx-auto">
        {/* <div className="container mx-auto px-4 sm:px-6 lg:px-8 flow-root"> */}
        <ul role="list" className="-mb-8">
          {messages.map((message, messageIndex) => (
            <li key={messageIndex}>
              <div className="relative pb-8">
                {messageIndex !== messages.length - 1 ? (
                  <span
                    className="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200"
                    aria-hidden="true"
                  />
                ) : null}
                <div className="relative flex space-x-3">
                  <div>
                    <span
                      className={
                        "h-8 w-8 rounded-full flex items-center justify-center ring-8 ring-white bg-green-400"
                      }
                    >
                      <MessageIcon />
                    </span>
                  </div>
                  <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                    <div>
                      <p className="text-sm text-gray-500">{message.content}</p>
                    </div>
                    {/* <div className="text-right text-sm whitespace-nowrap text-gray-500">
                  <time dateTime={event.datetime}>{event.date}</time>
                </div> */}
                  </div>
                </div>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

function MessageIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      className="h-5 w-5 text-white"
      aria-hidden="true"
      viewBox="0 0 20 20"
      fill="currentColor"
    >
      <path
        fillRule="evenodd"
        d="M18 5v8a2 2 0 01-2 2h-5l-5 4v-4H4a2 2 0 01-2-2V5a2 2 0 012-2h12a2 2 0 012 2zM7 8H5v2h2V8zm2 0h2v2H9V8zm6 0h-2v2h2V8z"
        clipRule="evenodd"
      />
    </svg>
  );
}
