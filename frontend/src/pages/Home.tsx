import { PinIcon } from "../components/Icons";

interface Concert {
  artist_name: string;
  event_name: string;
  venue: string;
  city: string;
  date: string;
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
        <ul className="concert-list">
          {concerts.map((c, i) => (
            <li key={i} className="concert-card" style={{ animationDelay: `${i * 0.05}s` }}>
              <div className="card-left">
                <div className="card-artist">{c.artist_name}</div>
                <div className="card-event">{c.event_name}</div>
                <div className="card-venue">
                  <PinIcon />
                  {c.venue}, {c.city}
                </div>
              </div>
              <div className="card-right">
                <div className="card-date">{formatDate(c.date)}</div>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
