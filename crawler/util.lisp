;;;; This is currently scratch space
(ql:quickload "cl-toml")
(ql:quickload "cl-ppcre")

(defun read-file (pathname)
  "Opens a file and returns its contents as a string"
  (with-open-file (fp pathname)
    (let ((data (make-array (file-length fp))))
      (coerce (subseq data 0 (read-sequence data fp)) 'string))))

;; This mess is temporary
(defvar *rules* (cl-toml:parse-file "../org.toml"))

(defun definitions (str)
  (let ((def (coerce (gethash "def" *rules*) 'list)))
    (flet ((trim (str) (string-left-trim (car def)
		       (string-right-trim (car (last def)) str))))
      (mapcar (lambda (sub) (cl-ppcre:split (caddr def) (trim sub)))
	      (cl-ppcre:all-matches-as-strings
	        (apply #'concatenate 'string def) str)))))

(defun quizletify (infile &optional (outfile nil))
  (let* ((defs (definitions (read-file infile)))
         (str (format nil "~:{~A	~A~%~}" defs)))
    (if outfile
        (with-open-file (o outfile :direction :output)
          (write-string str o))
        str)))
