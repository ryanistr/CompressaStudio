"""Extract text content from Materi/ lecture files (.pptx, .pdf) into Conclusion.md.

Requires:
  - markitdown (pip): python3 -m markitdown <file>
  - pdftotext (poppler-utils): pdftotext -layout <file> -
"""

import os
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
MATERI_DIR = SCRIPT_DIR / "Materi"
OUTPUT_FILE = SCRIPT_DIR / "Conclusion.md"


def extract_pptx(filepath: str) -> str:
    """Convert a .pptx file to markdown text via markitdown."""
    result = subprocess.run(
        [sys.executable, "-m", "markitdown", filepath],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(f"Warning: markitdown exited {result.returncode} for {filepath}", file=sys.stderr)
    if result.stderr:
        print(f"markitdown stderr for {filepath}: {result.stderr}", file=sys.stderr)
    return result.stdout


def extract_pdf(filepath: str) -> str:
    """Convert a .pdf file to plain text via pdftotext."""
    result = subprocess.run(
        ["pdftotext", "-layout", filepath, "-"],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(f"Warning: pdftotext exited {result.returncode} for {filepath}", file=sys.stderr)
    if result.stderr:
        print(f"pdftotext stderr for {filepath}: {result.stderr}", file=sys.stderr)
    return result.stdout


def main() -> None:
    if not MATERI_DIR.is_dir():
        print(f"Materi directory not found: {MATERI_DIR}", file=sys.stderr)
        sys.exit(1)

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        f.write("# Materi Contents\n\n")

        for filename in sorted(os.listdir(MATERI_DIR)):
            filepath = str(MATERI_DIR / filename)
            f.write(f"## {filename}\n\n")

            try:
                if filename.endswith(".pptx"):
                    f.write(extract_pptx(filepath))
                elif filename.endswith(".pdf"):
                    f.write(extract_pdf(filepath))
                else:
                    f.write(f"_Skipped: unsupported file type._\n")
            except Exception as exc:
                f.write(f"Error reading {filename}: {exc}\n")
                print(f"Exception processing {filename}: {exc}", file=sys.stderr)

            f.write("\n\n---\n\n")

    print(f"Extraction complete. Output: {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
