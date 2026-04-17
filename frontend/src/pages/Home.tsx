interface Concert {
  artist_name: string;
  event_name: string;
  venue: string;
  city: string;
  date: string;
  image_url: string | null;
}

function formatDate(iso: string): string {
  if (!iso || iso === "Unknown Date") return iso;
  const [year, month, day] = iso.split("-");
  const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
  return `${parseInt(day)} ${months[parseInt(month) - 1]} ${year}`;
}

interface Props {
  concerts: Concert[];
  country: string;
  onBack: () => void;
}

export default function Home({ concerts, country, onBack }: Props) {
  return (
    <div className="screen screen-results">
      <div className="results-header">
        <div>
          <h2 className="results-title">Upcoming concerts</h2>
          <p className="results-meta">
            {concerts.length === 0
              ? "No matches found"
              : `${concerts.length} show${concerts.length !== 1 ? "s" : ""} · ${country}`}
          </p>
        </div>
        <button className="btn-back" onClick={onBack}>
          ← Change country
        </button>
      </div>

      {concerts.length === 0 ? (
        <div className="empty">
          <div className="empty-icon">♩</div>
          <p className="empty-title">Nothing on for now</p>
          <p className="empty-sub">Try a different country or check back later.</p>
        </div>
      ) : (
        <div className="concert-grid">
          {concerts.map((c, i) => (
            <div
              key={i}
              className={`concert-card ${!c.image_url ? 'concert-card--no-image' : ''}`}
              style={{
                backgroundImage: c.image_url ? `url(${c.image_url})` : undefined,
                animationDelay: `${i * 0.05}s`,
              }}
            >
              <div className="concert-card__overlay" />
              <div className="concert-card__info">
                <span className="concert-card__artist">{c.artist_name}</span>
                <span className="concert-card__event">{c.event_name}</span>
                <span className="concert-card__venue">{c.venue}, {c.city}</span>
                <span className="concert-card__date">{formatDate(c.date)}</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
