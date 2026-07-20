import { invoke } from "@tauri-apps/api/core";

export type Note = {
  id: number;
  title: string;
  content: string;
  createdAt: string;
};

export async function createNote(
  title: string,
  content: string,
): Promise<number> {
  return await invoke<number>("create_note", {
    title,
    content,
  });
}

export async function getNotes(): Promise<Note[]> {
  return await invoke<Note[]>("get_notes");
}

export async function getNote(id: number): Promise<Note | null> {
  return await invoke<Note | null>("get_note", {
    id,
  });
}

export async function updateNote(
  id: number,
  title: string,
  content: string,
): Promise<boolean> {
  return await invoke<boolean>("update_note", {
    id,
    title,
    content,
  });
}

export async function deleteNote(id: number): Promise<boolean> {
  return await invoke<boolean>("delete_note", {
    id,
  });
}
