;;; slang_mode.el --- Major mode for Slang -*- lexical-binding: t; -*-

(require 'font-lock)

(defvar slang-mode-hook nil)

(defvar slang-mode-map
  (let ((map (define-keymap)))
    map)
  "Keymap for Slang major mode.")

(defconst slang-keywords
  '("if" "else" "while" "for" "return" "function" "fn" "test" "let" "import" "namespace"))

(defconst slang-types
  '("int" "float" "bool" "str"))

(defconst slang-builtins
  '("print" "first" "last" "len" "push" "rest" "Array" "File" "Fn" "HTTP" "Json" "Math" "Monad" "Option" "Result" "Object" "Regex" "String" "System" "Test" "Time" "Type"))

(defconst slang-font-lock-keywords
  `(
    ;; Keywords
    (,(regexp-opt slang-keywords 'symbols) . font-lock-keyword-face)

    ;; Types
    (,(regexp-opt slang-types 'symbols) . font-lock-type-face)

    ;; builtins
    (,(regexp-opt slang-builtins 'symbols) . font-lock-builtin-face)

    ;; function definitions
    ("\\<\\(fn\\|function\\)\\>\\s-+\\([a-zA-Z_][a-zA-Z0-9_]*\\)" 2 font-lock-function-name-face)

    ("\\<let\\s-+\\([a-zA-Z_][a-zA-Z0-9_]*\\)\\>" 1 font-lock-variable-name-face)

    ("\\<[0-9]+\\(\\.[0-9]+\\)?\\>" . font-lock-constant-face)
    ))

(defvar slang-mode-syntax-table
  (let ((st (make-syntax-table)))
    ;; // comments
    (modify-syntax-entry ?/ ". 124b" st)
    (modify-syntax-entry ?\n "> b" st)

    ;; strings
    (modify-syntax-entry ?\" "\"" st)
    st)
  "Syntax table for Slang mode.")

;;;###autoload
(define-derived-mode slang-mode prog-mode "Slang"
  "Major mode for editing Slang source code."
  :syntax-table slang-mode-syntax-table
  :keymap slang-mode-map

  (setq-local font-lock-defaults '(slang-font-lock-keywords))
  (setq-local indent-line-function #'slang-indent-line)
  (setq-local electric-indent-chars '(?\n ?\} ?\{))
  (setq-local imenu-generic-expression slang-imenu-generic-expression)

    ;; Optional: comment command support
  (setq-local comment-start "// ")
  (setq-local comment-end "")
  (imenu-add-menubar-index))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.sl\\'" . slang-mode))

(defcustom slang-indent-offset 4
  "Indentation width for Slang."
  :type 'integer
  :group 'slang)

(defun slang--previous-indentation ()
  "Return indentation of previous non-blank line."
  (save-excursion
    (forward-line -1)
    (while (and (not (bobp))
		(looking-at-p "^[[:space:]]*$"))
      (forward-line -1))
    (current-indentation)))

(defun slang-indent-line ()
  "Indent current line according to Slang rules."
  (interactive)
  (let ((indent 0)
        (pos (- (point-max) (point))))
    (save-excursion
      (beginning-of-line)

      ;; Dedent closing braces
      (cond
       ((looking-at-p "^[[:space:]]*}")
        (setq indent (max 0 (- (slang--previous-indentation)
                               slang-indent-offset))))

        ;; else aligns with if
	((looking-at-p "^[[:space:]]*else\\b")
	(setq indent (slang--previous-indentation)))
	
       ;; Normal lines
       (t
        (setq indent (slang--previous-indentation))
        (save-excursion
          (forward-line -1)
          (when (looking-at-p ".*{[[:space:]]*$")
            (setq indent (+ indent slang-indent-offset)))))))

    ;; Apply indentation
    (indent-line-to indent)

    ;; Restore point position
    (when (> (- (point-max) pos) (point))
      (goto-char (- (point-max) pos)))))

(defvar slang-imenu-generic-expression
  '(
    ("Functions"
     "\\<\\(fn\\|function\\)\\s-+\\([a-zA-Z_][a-zA-Z0-9_]*\\)\\>"
     2)

    ("Variables"
     "\\<let\\s-+\\([a-zA-Z_][a-zA-Z0-9_]*\\)\\>"
     1)

    ("Namespaces"
     "\\<namespace\\s-+\\([A-Z][a-zA-Z0-9_]*\\)\\>"
     1)
     ))
