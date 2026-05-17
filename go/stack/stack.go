package stack

import (
	"container/list"
	"errors"
)

type Stack struct {
	elements *list.List
}

func NewStack() *Stack {
	return &Stack{elements: list.New()}
}

func (s *Stack) Push(value any) {
	s.elements.PushBack(value)
}

func (s *Stack) Pop() (any, error) {
	if s.elements.Len() == 0 {
		return nil, errors.New("stack is empty")
	}

	element := s.elements.Back()
	s.elements.Remove(element)
	return element.Value, nil
}

func (s *Stack) Peek() (any, error) {
	if s.elements.Len() == 0 {
		return nil, errors.New("stack is empty")
	}
	return s.elements.Back().Value, nil
}

func (s *Stack) Len() int {
	return s.elements.Len()
}
