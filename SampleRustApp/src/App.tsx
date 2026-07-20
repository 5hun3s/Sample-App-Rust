import { FormEvent, useCallback, useEffect, useState } from "react";

import {
  createNote,
  deleteNote,
  getNotes,
  type Note,
} from "./services/noteService";

function App() {
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [notes, setNotes] = useState<Note[]>([]);
  const [errorMessage, setErrorMessage] = useState("");

  const loadNotes = useCallback(async (): Promise<void> => {
    try {
      const result = await getNotes();
      setNotes(result);
    } catch (error) {
      setErrorMessage(String(error));
    }
  }, []);

  useEffect(() => {
    void loadNotes();
  }, [loadNotes]);

  async function handleSubmit(
    event: FormEvent<HTMLFormElement>,
  ): Promise<void> {
    event.preventDefault();

    try {
      setErrorMessage("");

      await createNote(title, content);

      setTitle("");
      setContent("");

      await loadNotes();
    } catch (error) {
      setErrorMessage(String(error));
    }
  }

  async function handleDelete(id: number): Promise<void> {
    try {
      setErrorMessage("");

      await deleteNote(id);
      await loadNotes();
    } catch (error) {
      setErrorMessage(String(error));
    }
  }

  return (
    <main>
      <h1>SQLiteメモアプリ</h1>

      <form onSubmit={handleSubmit}>
        <div>
          <label htmlFor="title">タイトル</label>

          <input
            id="title"
            value={title}
            onChange={(event) => {
              setTitle(event.target.value);
            }}
          />
        </div>

        <div>
          <label htmlFor="content">内容</label>

          <textarea
            id="content"
            value={content}
            onChange={(event) => {
              setContent(event.target.value);
            }}
          />
        </div>

        <button type="submit">保存</button>
      </form>

      {errorMessage && <p role="alert">{errorMessage}</p>}

      <ul>
        {notes.map((note) => (
          <li key={note.id}>
            <h2>{note.title}</h2>
            <p>{note.content}</p>
            <small>{note.createdAt}</small>

            <div>
              <button
                type="button"
                onClick={() => {
                  void handleDelete(note.id);
                }}
              >
                削除
              </button>
            </div>
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;
