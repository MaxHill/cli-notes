package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path"
	"time"

	cal "github.com/MaxHill/cli-notes/calendar"
	"github.com/MaxHill/cli-notes/config"

	// "github.com/MaxHill/cli-notes/note"

	"github.com/MaxHill/cli-notes/note"
	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
)

// EVENT //

// MODEL //
type selectedEvent struct {
	set   bool
	value cal.Event
}

type model struct {
	events        list.Model
	newFileName   textinput.Model
	error         string
	selectedEvent selectedEvent
	notesRootDir  string
	createdNote   note.Note
}

func initList(rootDir string, events []cal.Event) model {
	var items []list.Item
	for _, event := range events {
		items = append(items, event)
	}

	const listHeight = 14
	const defaultWidth = 20
	eventList := list.New(items, list.NewDefaultDelegate(), defaultWidth, listHeight)
	eventList.Title = "Current meetings"

	ti := textinput.New()
	ti.Placeholder = "Some random meeting title"
	ti.Focus()
	ti.CharLimit = 156
	ti.Width = 20

	return model{
		events:        eventList,
		newFileName:   ti,
		error:         "",
		selectedEvent: selectedEvent{set: false},
		notesRootDir:  rootDir,
	}
}

// INIT //
func (m model) Init() tea.Cmd {
	// Just return `nil`, which means "no I/O right now, please."
	return nil
}

// UPDATE //
func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.events.SetWidth(msg.Width)
		return m, nil

	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "ctrl+c":
			return m, tea.Quit

		case "enter":
			e, ok := m.events.SelectedItem().(cal.Event)
			var n note.Note

			if ok {
				m.selectedEvent = selectedEvent{set: true, value: e}
				n = note.FromEvent(m.notesRootDir, e)
			} else if len(m.events.Items()) < 1 {
				if len(m.newFileName.Value()) < 1 {
					m.error = fmt.Sprintf("\nError: %s", "Please write a title for the meeting")
					return m, nil
				}
				t := time.Now()
				n = note.Note{
					Path:     m.notesRootDir,
					FileName: note.CreateFilename(t, t, m.newFileName.Value()),
					Content:  "",
				}

			}

			n.Save()
			m.createdNote = n

			return m, tea.Quit
		}
	}

	var cmd tea.Cmd
	if len(m.events.Items()) < 1 {
		m.newFileName, cmd = m.newFileName.Update(msg)
	} else {
		m.events, cmd = m.events.Update(msg)
	}
	return m, cmd
}

// VIEW //
func (m model) View() string {
	if len(m.events.Items()) < 1 {
		return fmt.Sprintf(
			"Title for the meeting:%s\n\n%s\n\n%s",
			m.error,
			m.newFileName.View(),
			"(esc to quit)",
		) + "\n"
	}
	return m.events.View()
}

func main() {
	// Flags
	subDir := flag.String("dir", "", "Subdirectory the note should be created in")
	flag.Parse()

	// Config
	conf, err := config.GetConfig()
	if err != nil {
		log.Fatal(err)
	}
	calendarRoot := conf.CalendarRoot
	notesRootDir := path.Join(conf.NotesRootDir, *subDir)

	// Program
	activeEvents := cal.ActiveEvents(cal.GetAllEvents(calendarRoot))
	m := initList(notesRootDir, activeEvents)

	p := tea.NewProgram(m, tea.WithOutput(os.Stderr))
	finalModel, err := p.StartReturningModel()
	if err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}

	// Output that can be piped
	if m, ok := finalModel.(model); ok {
		fmt.Println(m.createdNote.FullFile())
	}
}
