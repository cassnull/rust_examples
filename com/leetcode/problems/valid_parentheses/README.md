## 20\. Valid Parentheses

Given a string s containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

An input string is valid if:

1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.

**Example 1:**

<pre>
<strong>Input:</strong> s = "()"
<strong>Output:</strong> true
</pre>

**Example 2:**

<pre>
<strong>Input:</strong> s = "()[]{}"
<strong>Output:</strong> true
</pre>

**Example 3:**

<pre>
<strong>Input:</strong> s = "(]"
<strong>Output:</strong> false
</pre>

**Constraints:**

- <code>1 <= s.length <= 10<sup>4</sup></code>
- `s` consists of parentheses only `'()[]{}'`.