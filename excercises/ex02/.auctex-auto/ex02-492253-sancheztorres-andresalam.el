(TeX-add-style-hook
 "ex02-492253-sancheztorres-andresalam"
 (lambda ()
   (TeX-add-to-alist 'LaTeX-provided-package-options
                     '(("babel" "english") ("inputenc" "utf8") ("fontenc" "T1") ("geometry" "a4paper" "margin=1in") ("enumitem" "shortlabels")))
   (TeX-run-style-hooks
    "latex2e"
    "article"
    "art10"
    "babel"
    "inputenc"
    "fontenc"
    "geometry"
    "relsize"
    "amsfonts"
    "amsthm"
    "amssymb"
    "mathtools"
    "titlesec"
    "enumitem")
   (LaTeX-add-environments
    '("solution" LaTeX-env-args ["argument"] 0)))
 :latex)

