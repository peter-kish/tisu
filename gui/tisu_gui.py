import tkinter as tk
import json
import subprocess
from tkinter import ttk
from tkinter import filedialog as fd
from tkinter.messagebox import showinfo

app_state = {
    "tisu_path": "",
    "input_path": "",
    "filter_path": "",
    "output_path": "",
}


def save_app_state():
    # print(f"Saving app state:\n {app_state}")
    with open("tisu_gui.json", "w") as file:
        json.dump(app_state, file, indent=4)


def load_app_state():
    try:
        with open("tisu_gui.json", "r") as file:
            global app_state
            app_state = json.load(file)
        # print(f"Loaded app state:\n {app_state}")
    except Exception:
        print("Warning: Can't open tisu_gui.json")


load_app_state()


# create the root window
root = tk.Tk()
root.title("Tisu GUI")
root.resizable(True, True)
root.geometry("600x200")

def close_gui():
    save_app_state()
    root.destroy()

root.protocol("WM_DELETE_WINDOW", close_gui)


bin_filetypes = (
        ("Binary files", "*"),
        ("All files", "*.*")
    )
tmx_filetypes = (
        ("TMX files", "*.tmx"),
        ("All files", "*.*")
    )


def select_file(filetypes):
    return fd.askopenfilename(
        title="Input file",
        initialdir="/",
        filetypes=filetypes)


def select_tisu():
    filename = select_file(bin_filetypes)

    if not filename:
        return
    
    app_state["tisu_path"] = filename
    tisu_button.config(text=filename)


def select_input():
    filename = select_file(tmx_filetypes)

    if not filename:
        return
    
    app_state["input_path"] = filename
    input_button.config(text=filename)


def select_filter():
    filename = select_file(tmx_filetypes)

    if not filename:
        return
    
    app_state["filter_path"] = filename
    filter_button.config(text=filename)


def select_output():
    filename = select_file(tmx_filetypes)

    if not filename:
        return
    
    app_state["output_path"] = filename
    output_button.config(text=filename)


def run():
    for k,v in app_state.items():
        if not v:
            showinfo(
                title="Missing path",
                message=f"{k} missing",
                icon="error"
            )
            return
    print("Running tisu...")
    subprocess.run(get_cmd())
    print("...done.")
    close_gui()


def get_cmd():
    return [
        app_state["tisu_path"],
        "--input",
        app_state["input_path"],
        "--filters", app_state["filter_path"],
        "--output",
        app_state["output_path"]
        ]


tisu_frame = ttk.Frame(root)
tisu_frame.pack(side="top", fill="x")
input_frame = ttk.Frame(root)
input_frame.pack(side="top", fill="x")
filter_frame = ttk.Frame(root)
filter_frame.pack(side="top", fill="x")
output_frame = ttk.Frame(root)
output_frame.pack(side="top", fill="x")
button_frame = ttk.Frame(root)
button_frame.pack(side="bottom", expand=True, fill="both")


tisu_label = ttk.Label(tisu_frame, text="Tisu Executable:")
tisu_label.pack(side="left")

tisu_button = ttk.Button(
    tisu_frame,
    text=app_state["tisu_path"] if app_state["tisu_path"] else "...",
    command=select_tisu
)
tisu_button.pack(side="left", fill="x", expand=True)


input_label = ttk.Label(input_frame, text="Input:")
input_label.pack(side="left")

input_button = ttk.Button(
    input_frame,
    text=app_state["input_path"] if app_state["input_path"] else "...",
    command=select_input
)
input_button.pack(side="left", fill="x", expand=True)


filter_label = ttk.Label(filter_frame, text="Filter:")
filter_label.pack(side="left")

filter_button = ttk.Button(
    filter_frame,
    text=app_state["filter_path"] if app_state["filter_path"] else "...",
    command=select_filter
)
filter_button.pack(side="left", fill="x", expand=True)


output_label = ttk.Label(output_frame, text="Output:")
output_label.pack(side="left")

output_button = ttk.Button(
    output_frame,
    text=app_state["output_path"] if app_state["output_path"] else "...",
    command=select_output
)
output_button.pack(side="left", fill="x", expand=True)


run_button = ttk.Button(
    button_frame,
    text="Run",
    command=run
)
run_button.pack(fill="x", expand=False, side="bottom")
run_button.focus_set()


# Bind the Enter key to invoke the run_button
root.bind('<Return>', lambda event=None: run_button.invoke())


# run the application
root.mainloop()