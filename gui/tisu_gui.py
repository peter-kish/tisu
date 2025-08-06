import tkinter as tk
import json
import subprocess
from enum import Enum
from tkinter import ttk
from tkinter import filedialog as fd
from tkinter.messagebox import showinfo

APP_STATE_FILE = "tisu_gui.json"
FILETYPES_BIN = (
        ("Binary files", "*"),
        ("All files", "*.*")
    )
FILETYPES_TMX = (
        ("TMX files", "*.tmx"),
        ("All files", "*.*")
    )


class ParameterType(Enum):
    TISU = "tisu_path"
    INPUT = "input_path"
    FILTER = "filter_path"
    OUTPUT = "output_path"


PARAMETER_LABELS = {
    ParameterType.TISU: "Tisu Executable: ",
    ParameterType.INPUT: "Input: ",
    ParameterType.FILTER: "Filter: ",
    ParameterType.OUTPUT: "Output: ",
}

PARAMETER_FILE_TYPES = {
    ParameterType.TISU: FILETYPES_BIN,
    ParameterType.INPUT: FILETYPES_TMX,
    ParameterType.FILTER: FILETYPES_TMX,
    ParameterType.OUTPUT: FILETYPES_TMX,
}


class AppState:
    def __init__(self):
        self.paths = {}
        for parameter_type in ParameterType:
            self.paths[parameter_type.value] = ""

    def load(self):
        try:
            with open(APP_STATE_FILE, "r") as file:
                self.paths = json.load(file)
            # print(f"Loaded app state:\n {self.paths}")
        except Exception:
            print(f"Warning: Can't open {APP_STATE_FILE}")

    def save(self):
        # print(f"Saving app state:\n {self.paths}")
        with open(APP_STATE_FILE, "w") as file:
            json.dump(self.paths, file, indent=4)


class TisuGui:
    def __init__(self):
        self.app_state = AppState()
        self.app_state.load()
    
        self.root = TisuGui._create_root_widget()
        self.root.protocol("WM_DELETE_WINDOW", self._quit)

        self.param_buttons = {}
        for parameter_type in ParameterType:
            self.param_buttons[parameter_type] = self._create_parameter_button(parameter_type)

        self.run_button = self._create_run_button()

        # Bind the Enter key to invoke the run_button
        self.root.bind('<Return>', lambda event=None: self.run_button.invoke())

    def main_loop(self):
        self.root.mainloop()

    def _create_root_widget():
        root = tk.Tk()
        root.title("Tisu GUI")
        root.resizable(True, True)
        root.geometry("600x200")
        return root
    
    def _create_parameter_button(self, parameter_type):
        frame = ttk.Frame(self.root)
        frame.pack(side="top", fill="x")

        label = ttk.Label(frame, text=PARAMETER_LABELS[parameter_type])
        label.pack(side="left")

        button_text = self.app_state.paths[parameter_type.value]

        button = ttk.Button(
            frame,
            text=button_text if button_text else "...",
            command=lambda: self._select_file(parameter_type)
        )
        button.pack(side="left", fill="x", expand=True)
        return button
    
    def _select_file(self, parameter_type):
        filename = fd.askopenfilename(
            title="Select File",
            initialdir="/",
            filetypes=PARAMETER_FILE_TYPES[parameter_type])

        if not filename:
            return
        
        self.app_state.paths[parameter_type.value] = filename
        self.param_buttons[parameter_type].config(text=filename)

    def _create_run_button(self):
        frame = ttk.Frame(self.root)
        frame.pack(side="bottom", expand=True, fill="both")

        run_button = ttk.Button(
            frame,
            text="Run",
            command=self._run
        )
        run_button.pack(fill="x", expand=False, side="bottom")
        run_button.focus_set()
        return run_button

    def _run(self):
        for k,v in self.app_state.paths.items():
            if not v:
                showinfo(
                    title="Missing path",
                    message=f"Parameter '{k}' not set!",
                    icon="error"
                )
                return
        print("Running tisu...")
        subprocess.run(self._get_cmd())
        print("...done.")
        self._quit()

    def _get_cmd(self):
        return [
            self.app_state.paths[ParameterType.TISU.value],
            "--input",
            self.app_state.paths[ParameterType.INPUT.value],
            "--filters",
            self.app_state.paths[ParameterType.FILTER.value],
            "--output",
            self.app_state.paths[ParameterType.OUTPUT.value]
        ]
    
    def _quit(self):
        self.app_state.save()
        self.root.destroy()


TisuGui().main_loop()