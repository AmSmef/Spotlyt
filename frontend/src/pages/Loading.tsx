interface Props {
  dots: string;
}

export default function Loading({ dots }: Props) {
  return (
    <div className="screen screen-loading">
      <div className="loading-visual">
        {[0, 1, 2, 3, 4].map(i => (
          <div key={i} className="loading-bar" style={{ animationDelay: `${i * 0.12}s` }} />
        ))}
      </div>
      <p className="loading-text">Finding concerts{dots}</p>
      <p className="loading-sub">Checking Spotify · Querying Ticketmaster</p>
    </div>
  );
}
