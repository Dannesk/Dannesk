
// src/styles/theme.rs

pub const DARK_CSS: &str = r#"
:root, .theme-root {
    --bg-primary: #101217;
    --bg-secondary: #000000;     
    --bg-card: rgba(240, 248, 255, 0.04); 
    --bg-faint: rgba(240, 248, 255, 0.02);         
    --bg-grid: rgba(240, 248, 255, 0.02); 
    --border: rgba(240, 248, 255, 0.08);
    --input-border: rgba(240, 248, 255, 0.12);
    --text: #F0F8FF;
    --text-secondary: #F0F8FFBF;
    --btn: rgba(240, 248, 255, 0.06);
    --btn-hover: #F0F8FF;
    --btn-active: #F0F8FFA6;
    --brand-blue: #0066ff;
    --brand-blue-text: #0066ff;
    --accent: #F0F8FFD9;
    --selection: rgba(240, 248, 255, 0.12);
    --input-bg: rgba(0, 0, 0, 0.25);
    --focus-ring: #0066ff;
    --status-ok: #10b981;
    --status-warn: #ef4444;
    --alice-blue-transparency-60: #F0F8FF99;
    --alice-blue-transparency-30: #F0F8FF4D;
}
.theme-root {
    background: var(--bg-primary);
    color: var(--text);
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.5;
}
.monospace-data { font-family: 'JetBrains Mono', monospace; }

.theme-btn {
    background: var(--btn);
    color: var(--text);
    border: none;
    padding: 10px 20px;
    font-weight: 600;
    font-size: 0.9rem;
    border-radius: 8px;
    transition: background 0.15s;
}
.theme-btn:hover {
    background: var(--btn-hover);
    color: var(--bg-primary);
}
"#;


pub const LIGHT_CSS: &str = r#"
:root, .theme-root {
    /* Backgrounds: Using Alice Blue as the base canvas */
    --bg-primary: #F0F8FF;       
    --bg-secondary: #FFFFFF;     
    --bg-card: #FFFFFF;          
    --bg-faint: rgba(18, 20, 25, 0.03); /* Subtle hint of brand dark */
    --bg-grid: rgba(18, 20, 25, 0.05); 

    /* Borders & Dividers: Brand Dark with opacity */
    --border: rgba(18, 20, 25, 0.1);           
    --input-border: rgba(18, 20, 25, 0.2);     

    /* Text: Brand Dark Background color used as primary text */
    --text: #121419;             
    --text-secondary: rgba(18, 20, 25, 0.6);   
    --text-accent: #121419;      

    /* Interactive: Brand Dark and Brand Blue */
    --btn: #121419;              
    --btn-hover: rgba(18, 20, 25, 0.8);        
    --btn-active: #121419;       
    --accent: #121419;           
    
    --selection: rgba(0, 102, 255, 0.1);
    --brand-blue: #121419;
    --brand-blue-text: #121419;

    --input-bg: #FFFFFF;
    --focus-ring: #121419;       
    
    /* Status: Kept functional */
    --status-ok: #16A34A;        
    --status-warn: #ef4444;  
    
    --alice-blue-transparency-60: #F0F8FF99
    --alice-blue-transparency-30: #F0F8FF4D
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

.theme-btn:hover {
    background: var(--text);
    color: var(--bg-primary);
}
"#;