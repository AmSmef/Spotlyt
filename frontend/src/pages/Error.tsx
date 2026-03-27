interface Props {
  message: string;
  onBack: () => void;
}

export default function Error({ message, onBack }: Props) {
  return (
    <div className="screen screen-error">
      <h2 className="error-title">Something went wrong</h2>
      <p className="error-msg">{message}</p>
      <button className="btn-back" onClick={onBack}>← Try again</button>
    </div>
  );
}
