import { Link } from "@tanstack/react-router";

const Header = () => (
  <header className="p-2 flex gap-2 bg-white text-black justify-between">
    <nav className="flex flex-row">
      <div className="px-2 font-bold">
        <Link to="/">Home</Link>
      </div>
    </nav>
  </header>
);

export { Header };
