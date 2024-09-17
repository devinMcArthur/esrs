class LoadingSpinner extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    this.render();
  }

  static get observedAttributes() {
    return ['color', 'size'];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === 'color' && oldValue !== newValue) {
      this.render();
    }

    if (name === 'size' && oldValue !== newValue) {
      this.render();
    }
  }

  render() {
    let color = this.getAttribute('color') || '#f97316';
    let size = this.getAttribute('size') || '50';
    this.shadowRoot.innerHTML = `
      <style> 
        @keyframes spin { 
          from { transform: rotate(0deg); } 
          to { transform: rotate(360deg); } 
        } 
        @keyframes reverse-spin { 
          from { transform: rotate(0deg); } 
          to { transform: rotate(-360deg); } 
        } 
        .spinner, .small-spinner { 
          transform-origin: center; 
        } 
        .spinner { 
          animation: spin 3s linear infinite; 
        } 
        .small-spinner { 
          animation: reverse-spin 2s linear infinite; 
        } 
      </style> 
      <svg width="${size}" height="${size}" viewBox="0 0 50 50"> 
        <circle class="spinner" cx="25" cy="25" r="20" fill="none" stroke-width="5" stroke="${color}" 
          stroke-dasharray="94.2" stroke-dashoffset="0" stroke-linecap="round" /> 
        <circle class="small-spinner" cx="25" cy="25" r="10" fill="none" stroke-width="3" stroke="${color}" 
          stroke-dasharray="47.1" stroke-dashoffset="0" stroke-linecap="round" /> 
      </svg> 
    `;
  }
}

customElements.define('loading-spinner', LoadingSpinner);
