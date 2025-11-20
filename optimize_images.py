import os
from pathlib import Path
from PIL import Image
import sys

def optimize_images(directory):
    """
    Recursively finds images in directory and converts them to WebP.
    """
    print(f"🔍 Scanning {directory} for images...")
    
    count = 0
    saved_space = 0
    
    for root, _, files in os.walk(directory):
        for file in files:
            if file.lower().endswith(('.png', '.jpg', '.jpeg')):
                file_path = Path(root) / file
                webp_path = file_path.with_suffix('.webp')
                
                # Skip if WebP already exists and is newer
                if webp_path.exists() and webp_path.stat().st_mtime > file_path.stat().st_mtime:
                    continue
                    
                try:
                    with Image.open(file_path) as img:
                        # Convert to RGB if necessary (e.g. PNG with transparency)
                        if img.mode in ('RGBA', 'LA'):
                            # Keep transparency for PNGs
                            pass
                        elif img.mode == 'P':
                            img = img.convert('RGBA')
                            
                        # Save as WebP
                        print(f"⚡ Converting: {file} -> .webp")
                        img.save(webp_path, 'WEBP', quality=85, method=6)
                        
                        # Calculate savings
                        original_size = file_path.stat().st_size
                        webp_size = webp_path.stat().st_size
                        saved = original_size - webp_size
                        
                        if saved > 0:
                            saved_space += saved
                            print(f"   📉 Saved {saved / 1024:.1f} KB ({(saved/original_size)*100:.1f}%)")
                        else:
                            print(f"   ⚠️  WebP was larger, keeping original but WebP exists for support.")
                            
                        count += 1
                        
                except Exception as e:
                    print(f"❌ Error converting {file}: {e}")

    print(f"\n✨ Optimization Complete!")
    print(f"🖼️  Converted {count} images")
    print(f"💾 Total space saved: {saved_space / 1024 / 1024:.2f} MB")

if __name__ == "__main__":
    # Check for Pillow
    try:
        import PIL
    except ImportError:
        print("❌ Pillow library not found. Please run: pip install Pillow")
        sys.exit(1)

    # Directories to scan
    dirs = ['static/images', 'content']
    
    for d in dirs:
        if os.path.exists(d):
            optimize_images(d)
        else:
            print(f"⚠️  Directory not found: {d}")
