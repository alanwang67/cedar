template(?folder: FS::Folder) =>
permit(
  principal == ?principal,
  action,
  resource in ?resource)
when {
    // take any action in the scope that you're admin on
    resource in ?folder
        
    // allow access up the tree for navigation
    || (action == Action::Navigate && ?folder in resource)
};

template(?date: datetime) => 
permit(
  principal == ?principal, 
  action == Action::"Access",
  resource is Company::InternalServices
) when {
    context.date > ?date 
};