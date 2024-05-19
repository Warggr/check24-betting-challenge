export type User = {
  username: string;
};

export type Session = {
  id: string;
  validUntil: string;
  user: User;
  authorizer: string;
  miniSession: boolean;
};
