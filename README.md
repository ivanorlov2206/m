# M: tool for storing useful commands

This console tool allows you to easily store the bash commands with meaningful descriptions for each of them.
Basically, it generates a markdown document for you which you can view or update on your own.

![hippo](../assets/m.gif)

## Installation

```
cargo build
echo -e "\nexport PATH=\$PATH:$(pwd)/target/debug/\n" >> ~/.bashrc

echo -e "\nexport M_DOCPATH=<your folder for the document>\n" >> ~/.bashrc
```

## Usage

You can save a new command to the document using:

```
m r <your command>
```

This will open your favourite text editor (which could be set with `M_EDITOR` env var, vim by default), so you
can edit the command description before saving it to the doc.

You can also pass the command from stdin, using

```
echo <command> | m i
```

To view/edit the doc promptly, run

```
m d
```

## Binding for tmux

You can add the following configuration to your tmux config `~/.tmux.conf`:

```
bind-key -T copy-mode-vi P send -X copy-pipe-and-cancel 'bash -c "
read -r -d \"\" sel
tmux set-buffer -- \"$sel\"
tmux save-buffer /tmp/m-select
tmux send-keys -X cancel
tmux new-window
tmux send-keys \"cat /tmp/m-select | m i\" C-m
tmux delete-buffer
"'
```

Now, if you press capital `P` in the VI tmux mode, the M tool will be open automatically for you with the selected
text as the target command.
