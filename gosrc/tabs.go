package main

import (
	"fmt"
	"os"
	"path"
	"strconv"
	"strings"
)

const NUM_ARGS = 4
const DEBUG_BUFFER = "*debug*"
const ELLIPSES = "…"
const FOCUSED_FMT = "{Prompt}"
const OTHER_FMT = "{LineNumbers}"
const SEPARATOR_FMT = "{Default}"

func printUsage() {
	fmt.Println(`usage: tabs <cols> <percentage> <separator> <focused> <buf1 buf2 ...>
	cols: the number of columns in the terminal
	percentage: how much of the width of the terminal should be used by tabs (0-100)
	separator: single character used to separate tabs
	focused: name of the buffer that is currently focused
	buf1 buf2 ...: list of buffers`)
}

type Args struct {
	// columns is how many columns the tab bar will take up
	columns int

	// separator is the string separating tabs
	separator string

	// focused is the index of the focused buffer within buffers
	focused int

	// buffers is the list of buffers
	buffers []string

	// hidden is the number of hidden buffers
	hidden int

	// debug represents whether or not we are on the debug buffer
	debug bool
}

// parse parses the provided positional arguments
func (a *Args) parse(args []string) error {
	if len(args) < NUM_ARGS {
		return fmt.Errorf("not enough args")
	}

	// compute available columns
	columns, err := strconv.Atoi(args[0])
	if err != nil {
		return fmt.Errorf("parse columns: %v", err)
	}
	percentage, err := strconv.Atoi(args[1])
	if err != nil {
		return fmt.Errorf("parse percentage: %v", err)
	}
	a.columns = columns * percentage / 100

	a.separator = args[2]

	// compute basenames of buffers, removing *debug* if we are not on it
	focused := args[3]
	a.buffers = make([]string, 0, len(args[4:]))
	for _, buf := range args[4:] {
		if buf == focused {
			a.focused = len(a.buffers)
			a.debug = buf == DEBUG_BUFFER
		}

		if !a.debug && buf == DEBUG_BUFFER {
			a.hidden += 1
		} else {
			a.buffers = append(a.buffers, path.Base(buf))
		}
	}

	return nil
}

// len computes the length of modelinefmt if no shortening is done
func (a *Args) len() int {
	buflens := 0
	for _, buf := range a.buffers {
		buflens += len(buf)
	}

	seps := (len(a.buffers) + 1) * len(a.separator)
	spaces := len(a.buffers) * 2

	return buflens + seps + spaces
}

// modelinefmt computes modelinefmt, using a compact representation if there are too many buffers
func (a *Args) modelinefmt() string {
	if a.len() > a.columns {
		return a.modelinefmtCompact()
	}

	return a.modelinefmtFull()
}

// modelinefmtFull computes modelinefmt without any shortening
func (a *Args) modelinefmtFull() string {
	var modelinefmt strings.Builder

	// constructs modelinefmt slice like []string{"|", " ", "{LineNumbers}", "bufname.txt", ...}
	modelinefmt.WriteString(a.separator)
	for i, buf := range a.buffers {
		modelinefmt.WriteString(" ")
		if i == a.focused {
			modelinefmt.WriteString(FOCUSED_FMT)
		} else {
			modelinefmt.WriteString(OTHER_FMT)
		}
		modelinefmt.WriteString(buf)
		modelinefmt.WriteString(" ")

		modelinefmt.WriteString(SEPARATOR_FMT)
		modelinefmt.WriteString(a.separator)
	}

	return modelinefmt.String()
}

// modelinefmtCompact computes modelinefmt without exceeding a.columns in effective length
func (a *Args) modelinefmtCompact() string {
	var modelinefmt strings.Builder

	// compute available space for non-focused buffer names
	seps := (len(a.buffers) + 1) * len(a.separator)
	spaces := len(a.buffers) * 2

	otherBufsAvailableSpace := a.columns - seps - spaces - len(a.buffers[a.focused])

	spacePerBuf := otherBufsAvailableSpace / (len(a.buffers) - 1)
	spacePerBufRem := otherBufsAvailableSpace % (len(a.buffers) - 1)

	modelinefmt.WriteString(a.separator)
	for i, buf := range a.buffers {
		modelinefmt.WriteString(" ")
		if i == a.focused {
			modelinefmt.WriteString(FOCUSED_FMT)
		} else {
			modelinefmt.WriteString(OTHER_FMT)
			if len(buf) > spacePerBuf {
				// compute how much space will be allocated for this buffer, potentially adding additional space if available
				space := spacePerBuf
				if spacePerBufRem > 0 {
					space++
					spacePerBufRem--
				}

				// if we don't have enough space, trim the buffer
				if space <= 0 {
					buf = ""
				} else if len(buf) > space {
					buf = buf[:space-1] + ELLIPSES
				}
			}
		}

		modelinefmt.WriteString(buf)
		modelinefmt.WriteString(" ")

		modelinefmt.WriteString(SEPARATOR_FMT)
		modelinefmt.WriteString(a.separator)
	}

	return modelinefmt.String()
}

func main() {
	var args Args
	if err := args.parse(os.Args[1:]); err != nil {
		fmt.Printf("%v\n", err)
		printUsage()
		os.Exit(1)
	}

	fmt.Println(args.modelinefmt())
}
