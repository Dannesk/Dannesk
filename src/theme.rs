// src/styles/theme.rs

pub const DARK_CSS: &str = r#"
:root, .theme-root {
    --bg-primary: #0D1111;
    --bg-secondary: #000000;     
    --bg-card: #141414;
    --bg-faint: #1c1c1c;         
    --bg-grid: rgba(226, 232, 240, 0.02); 
    --border: #262626;
    --input-border: rgba(226, 232, 240, 0.1);
    --text: #E2E8F0;
    --text-secondary: #94A3B8;
    --btn: #141414;
    --btn-hover: #E2E8F0;
    --btn-active: #CBD5E1;
    --accent: #E2E8F0;
    --selection: rgba(226, 232, 240, 0.1);
    --input-bg: #0a0a0a;
    --focus-ring: #475569;
    --status-ok: #10b981;
    --status-warn: #ef4444;
}
.theme-root {
    background: var(--bg-primary);
    color: var(--text);
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
.market-value, .monospace-data { font-family: 'JetBrains Mono', monospace; }

.theme-btn {
    background: transparent; /* Force transparency as requested */
    color: var(--text);
    border: 1px solid var(--border);
    padding: 6px 12px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.1em;
}
"#;

pub const LIGHT_CSS: &str = r#"
:root, .theme-root {
    /* Backgrounds: Logo Background Base */
    --bg-primary: #F8FAFC;       
    --bg-secondary: #FFFFFF;     
    --bg-card: #FFFFFF;          
    --bg-faint: #F1F5F9;         
    --bg-grid: rgba(13, 17, 17, 0.02); 

    /* Borders & Dividers: Neutral Slate */
    --border: #E2E8F0;           
    --input-border: #CBD5E1;     

    /* Text: Obsidian Logo Primary */
    --text: #0D1111;             
    --text-secondary: #64748B;   
    --text-accent: #334155;      

    /* Interactive: Obsidian Theme */
    --btn: #0D1111;              
    --btn-hover: #334155;        
    --btn-active: #1A1F1F;       
    --accent: #0D1111;           
    
    --selection: #E2E8F0;
    
    --input-bg: #FFFFFF;
    --focus-ring: #94A3B8;       
    
    /* Status */
    --status-ok: #16A34A;        
    --status-warn: #DC2626;      
}

.theme-root {
    background: var(--bg-primary);
    color: var(--text);
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.market-value, .monospace-data { 
    font-family: 'JetBrains Mono', monospace; 
}

.theme-btn {
    background: transparent;
    color: var(--text);
    border: 1px solid var(--border);
    padding: 6px 12px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-size: 11px;
    transition: all 0.2s ease;
}
"#;