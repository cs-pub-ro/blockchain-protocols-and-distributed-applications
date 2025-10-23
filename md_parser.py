#!/usr/bin/env python3
"""
Markdown parser for quizify plugin.
Parses quiz markdown files and returns structured quiz data.
"""

import re
import os


def parse_quiz(content, **kwargs):
    """
    Parse quiz content from markdown format.
    
    Expected format:
    # Question Title
    
    ## Question Text
    Question text here
    
    ## Question Answers
    - Option 1
    + Correct Option
    - Option 3
    - Option 4
    
    ## Feedback
    Feedback text here
    
    Args:
        content (str): The markdown content of the quiz file
        **kwargs: Additional keyword arguments
        
    Returns:
        dict: Dictionary containing quiz data with keys:
            - question: The question text
            - wrong: List of wrong answers
            - answer: The correct answer
            - feedback: Optional feedback text
    """
    
    # Extract question title
    title_match = re.search(r'^# (.+)$', content, re.MULTILINE)
    title = title_match.group(1) if title_match else "Quiz Question"
    
    # Extract question text
    question_match = re.search(r'## Question Text\s*\n(.+?)(?=\n##|\Z)', content, re.DOTALL)
    question_text = question_match.group(1).strip() if question_match else ""
    
    # Extract answers
    answers_match = re.search(r'## Question Answers\s*\n(.+?)(?=\n##|\Z)', content, re.DOTALL)
    answers_section = answers_match.group(1) if answers_match else ""
    
    # Parse answers
    wrong_answers = []
    correct_answer = ""
    
    if answers_section:
        answer_lines = answers_section.strip().split('\n')
        for line in answer_lines:
            line = line.strip()
            if line.startswith('+'):
                # Correct answer
                correct_answer = line[1:].strip()
            elif line.startswith('-'):
                # Wrong answer
                wrong_answers.append(line[1:].strip())
    
    # Extract feedback
    feedback_match = re.search(r'## Feedback\s*\n(.+?)(?=\n##|\Z)', content, re.DOTALL)
    feedback = feedback_match.group(1).strip() if feedback_match else ""
    
    # Combine title and question text
    full_question = f"{title}\n\n{question_text}".strip()
    
    return {
        'question': full_question,
        'wrong': wrong_answers,
        'answer': correct_answer,
        'feedback': feedback
    }


def main():
    """Test the parser with sample content."""
    sample_content = """# Test Question

## Question Text

What is the capital of France?

## Question Answers

- London
- Berlin
+ Paris
- Madrid

## Feedback

Paris is the capital and largest city of France.
"""
    
    result = parse_quiz(sample_content)
    print("Parsed quiz data:")
    print(result)


if __name__ == "__main__":
    main()
