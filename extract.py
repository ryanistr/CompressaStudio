import os
import subprocess

materi_dir = '/mnt/project/kompresidata/Materi'
output_file = '/mnt/project/kompresidata/Conclusion.md'

with open(output_file, 'w') as f:
    f.write('# Materi Contents\n\n')
    
    for filename in sorted(os.listdir(materi_dir)):
        filepath = os.path.join(materi_dir, filename)
        f.write(f'## {filename}\n\n')
        
        if filename.endswith('.pptx'):
            try:
                result = subprocess.run(['python', '-m', 'markitdown', filepath], capture_output=True, text=True)
                f.write(result.stdout)
                if result.stderr:
                    print(f"Error in {filename}: {result.stderr}")
            except Exception as e:
                f.write(f'Error reading {filename}: {e}\n')
        elif filename.endswith('.pdf'):
            try:
                result = subprocess.run(['pdftotext', '-layout', filepath, '-'], capture_output=True, text=True)
                f.write(result.stdout)
                if result.stderr:
                    print(f"Error in {filename}: {result.stderr}")
            except Exception as e:
                f.write(f'Error reading {filename}: {e}\n')
                
        f.write('\n\n---\n\n')

print("Extraction complete.")
