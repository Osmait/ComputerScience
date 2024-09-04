package main

import (
	"errors"
	"fmt"
)

func main() {
	list := newSiglyLinkedList[int]()
	list.AddAtBeg(1)

	list.AddAtBeg(2)
	list.Display()
}

type Node[T any] struct {
	Val  T
	Prev *Node[T]
	Next *Node[T]
}

func NewNode[T any](val T) *Node[T] {
	return &Node[T]{val, nil, nil}
}

type SinglyLinkedList[T any] struct {
	head   *Node[T]
	length int
}

func newSiglyLinkedList[T any]() *SinglyLinkedList[T] {
	return &SinglyLinkedList[T]{}
}

func (s *SinglyLinkedList[T]) AddAtBeg(val T) {
	n := NewNode(val)
	n.Next = s.head
	s.head = n
	s.length++
}

func (s *SinglyLinkedList[T]) AddAtEnd(val T) {
	n := NewNode(val)
	if s.head == nil {
		s.head = n
		s.length++
		return
	}
	cur := s.head
	for ; cur.Next != nil; cur = cur.Next {
	}
	cur.Next = n
	s.length++
}

func (s *SinglyLinkedList[T]) DelAtBeg() (T, bool) {
	if s.head == nil {
		var r T
		return r, false
	}
	cur := s.head
	s.head = cur.Next
	s.length--
	return cur.Val, true
}

func (s *SinglyLinkedList[T]) DelAtEnd() (T, bool) {
	if s.head == nil {
		var r T
		return r, false
	}
	if s.head.Next == nil {
		return s.DelAtBeg()
	}
	cur := s.head
	for ; cur.Next.Next != nil; cur = cur.Next {
	}
	retVal := cur.Next.Val
	cur.Next = nil
	s.length--
	return retVal, true
}

func (s *SinglyLinkedList[T]) DelByPos(pos int) (T, bool) {
	switch {
	case s.head == nil:
		var r T
		return r, false
	case pos-1 > s.length:
		var r T
		return r, false
	case pos-1 == 0:
		return s.DelAtBeg()
	case pos-1 == s.length:
		return s.DelAtEnd()
	}
	var prev *Node[T]
	var val T
	cur := s.head
	count := 0

	for count < pos-1 {
		prev = cur
		cur = cur.Next
		count++
	}
	val = cur.Val
	prev.Next = cur.Next
	s.length--
	return val, true
}

func (s *SinglyLinkedList[T]) Count() int {
	return s.length
}

func (s *SinglyLinkedList[T]) Reverse() {
	var prev, Next *Node[T]
	cur := s.head
	for cur != nil {
		Next = cur.Next
		cur.Next = prev
		prev = cur
		cur = Next
	}
	s.head = prev
}

func (s *SinglyLinkedList[T]) ReversePartition(left, right int) error {
	err := s.checkPartition(left, right)
	if err != nil {
		return err
	}
	tmpNode := &Node[T]{}
	tmpNode.Next = s.head
	pre := tmpNode
	for i := 0; i < left-1; i++ {
		pre = pre.Next
	}
	cur := pre.Next
	for i := 0; i < right-left; i++ {
		next := cur.Next
		cur.Next = next.Next
		next.Next = pre.Next
		pre.Next = next
	}
	s.head = tmpNode.Next
	return nil
}

func (s *SinglyLinkedList[T]) checkPartition(left, right int) error {
	if left > right {
		return errors.New("left boundary must smaller than right")
	} else if left < 1 {
		return errors.New("left boundary starts from the first node")
	} else if right > s.length {
		return errors.New("right boundary cannot be greater than the length of the linked list")
	}
	return nil
}

func (s *SinglyLinkedList[T]) Display() {
	for cur := s.head; cur != nil; cur = cur.Next {
		fmt.Print(cur.Val, "<-")
	}

	fmt.Print("\n")
}
