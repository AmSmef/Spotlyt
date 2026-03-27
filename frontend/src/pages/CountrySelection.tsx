import React from "react";

const COMMON_COUNTRIES = ["GB", "US", "DE", "FR", "AU", "IE", "NL"];

interface Props {
  country: string;
  onCountryChange: (value: string) => void;
  onFetch: () => void;
}

export default function CountrySelection({ country, onCountryChange, onFetch }: Props) {
  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter") onFetch();
  }

  return (
    <div className="screen screen-country">
      <div className="step-tag">Step 2 of 2</div>
      <h2 className="country-title">Where are you?</h2>
      <p className="country-sub">Enter a two-letter country code to filter concerts near you.</p>

      <div className="input-group">
        <input
          className="country-input"
          type="text"
          maxLength={2}
          placeholder="GB"
          value={country}
          onChange={e => onCountryChange(e.target.value.toUpperCase())}
          onKeyDown={handleKeyDown}
          autoFocus
          spellCheck={false}
        />
        <button
          className="btn-find"
          onClick={onFetch}
          disabled={country.trim().length !== 2}
        >
          Find concerts
        </button>
      </div>

      <div className="chips">
        {COMMON_COUNTRIES.map(c => (
          <button
            key={c}
            className={`chip ${country === c ? "chip-active" : ""}`}
            onClick={() => onCountryChange(c)}
          >
            {c}
          </button>
        ))}
      </div>
    </div>
  );
}
